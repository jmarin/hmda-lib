//See https://www.consumerfinance.gov/eregulations/1003-C/2015-26607_20180101#1003-C-1

pub fn check_digit() {}

fn calculate_mod(i: u64) -> u64 {
    i % 97
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_calculate_mod() {
        assert_eq!(calculate_mod(100), 3);
    }

}
