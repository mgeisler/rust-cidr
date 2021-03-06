use bitstring::*;
use std::cmp::min;
use std::fmt;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

use super::{Ipv4Inet, Ipv6Inet};
use super::from_str::inet_from_str;
use super::super::cidr::*;
use super::super::errors::*;
use super::super::family::Family;
use super::super::traits::*;

macro_rules! impl_inet_for {
	($n:ident : cidr $cidr:ident : addr $addr:ty : family $family:expr) => (
		impl BitString for $n {
			fn get(&self, ndx: usize) -> bool {
				self.address.get(ndx)
			}

			fn set(&mut self, ndx: usize, bit: bool) {
				assert!(ndx < self.network_length as usize);
				self.address.set(ndx, bit);
			}

			fn flip(&mut self, ndx: usize) {
				assert!(ndx < self.network_length as usize);
				self.address.flip(ndx);
			}

			fn len(&self) -> usize {
				self.network_length as usize
			}

			fn clip(&mut self, len: usize) {
				if len > 255 { return; }
				self.address.set_false_from(len);
				self.network_length = min(self.network_length, len as u8);
			}

			fn append(&mut self, bit: bool) {
				self.address.set(self.network_length as usize, bit);
				self.network_length += 1;
			}

			fn null() -> Self {
				$n{
					address: FixedBitString::new_all_false(),
					network_length: 0,
				}
			}

			fn shared_prefix_len(&self, other: &Self) -> usize {
				let max_len = min(self.network_length, other.network_length) as usize;
				FixedBitString::shared_prefix_len(&self.address, &other.address, max_len)
			}
		}

		impl Inet for $n {
			type Address = $addr;
			type Cidr = $cidr;

			fn new(
				addr: Self::Address,
				len: u8,
			) -> Result<Self, NetworkLengthTooLongError> {
				if len > $family.len() {
					Err(NetworkLengthTooLongError::new(len as usize, $family).into())
				} else {
					Ok($n{
						address: addr,
						network_length: len,
					})
				}
			}

			fn new_host(addr: Self::Address) -> Self {
				$n{
					address: addr,
					network_length: $family.len(),
				}
			}

			fn next(&mut self) -> bool {
				self.address.inc(self.network_length as usize)
			}

			fn network(&self) -> Self::Cidr {
				$cidr::new(self.first_address(), self.network_length).unwrap()
			}

			fn address(&self) -> Self::Address {
				self.address.clone()
			}

			fn first_address(&self) -> Self::Address {
				let mut a = self.address.clone();
				a.set_false_from(self.network_length as usize);
				a
			}

			fn first(&self) -> Self {
				$n{
					address: self.first_address(),
					network_length: self.network_length,
				}
			}

			fn last_address(&self) -> Self::Address {
				let mut a = self.address.clone();
				a.set_true_from(self.network_length as usize);
				a
			}

			fn last(&self) -> Self {
				$n{
					address: self.last_address(),
					network_length: self.network_length,
				}
			}

			fn network_length(&self) -> u8 {
				self.network_length
			}

			fn family(&self) -> Family {
				$family
			}

			fn mask(&self) -> Self::Address {
				let mut a = Self::Address::new_all_true();
				a.set_false_from(self.network_length as usize);
				a
			}

			fn contains(&self, addr: &Self::Address) -> bool {
				self.address.contains(self.network_length as usize, addr)
			}
		}

		impl fmt::Debug for $n {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				write!(f, "{:?}/{}", self.address, self.network_length)
			}
		}

		impl fmt::Display for $n {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				write!(f, "{}", self.address)?;
				if self.network_length != $family.len() {
					write!(f, "/{}", self.network_length)?;
				}
				Ok(())
			}
		}

		impl FromStr for $n {
			type Err = NetworkParseError;
			fn from_str(s: &str) -> Result<$n, NetworkParseError> {
				inet_from_str(s)
			}
		}
	)
}

impl_inet_for!{Ipv4Inet : cidr Ipv4Cidr : addr Ipv4Addr : family Family::Ipv4}
impl_inet_for!{Ipv6Inet : cidr Ipv6Cidr : addr Ipv6Addr : family Family::Ipv6}
