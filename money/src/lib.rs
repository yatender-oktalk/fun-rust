#[derive(PartialEq, Debug)]
pub struct USD(i32);
#[derive(PartialEq, Debug)]
pub struct GBP(i32);
#[derive(PartialEq, Debug)]
pub struct INR(i32);

pub trait ToUSD {
    fn to_usd(&self) -> USD;
}

impl ToUSD for INR {
    fn to_usd(&self) -> USD {
        USD((self.0 * 5) / 100)
    }
}

impl ToUSD for GBP {
    fn to_usd(&self)-> USD {
        USD((self.0 * 130) / 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let g  = GBP(200);
        let u = g.to_usd();
        assert_eq!(u, USD(260));

        let inr  = INR(7500);
        let usd = inr.to_usd();
        assert_eq!(usd, USD(375));
    }
}
