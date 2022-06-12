use serde::{de, Deserialize, Deserializer, Serialize};

use crate::orderbook_aggregator::{Level, Summary};


pub trait Order {
    fn exchange_name(&self) -> String;
    fn asks(&self) -> &Vec<Quote>;
    fn bids(&self) -> &Vec<Quote>;
}

#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub struct Quote {
    #[serde(deserialize_with = "from_str")]
    pub price: f64,
    #[serde(deserialize_with = "from_str")]
    pub amount: f64,
}

fn from_str<'a, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'a>
{
    let string_num: &str = Deserialize::deserialize(deserializer)?;
    string_num.parse::<f64>().map_err(de::Error::custom)
}

pub fn summarise_order<T: Order>(order: T) -> Summary {
    let mut asks = Vec::new();
    let mut bids = Vec::new();

    for (ask, bid) in
        order.asks().iter().zip(order.bids().iter()) {
        let ask_level = Level {
            exchange: order.exchange_name(),
            amount: ask.amount,
            price: ask.price,
        };

        asks.push(ask_level);

        let bid_level = Level {
            exchange: order.exchange_name(),
            amount: bid.amount,
            price: bid.price,
        };
        bids.push(bid_level);
    }

    let spread = bids[0].price - asks[0].price;
    Summary {
        asks,
        bids,
        spread,
    }
}


#[test]
fn test_deserialize_from_str_succeeds() {
    use serde_json::json;

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    pub struct TestQuote {
    #[serde(deserialize_with = "from_str")]
    pub price: f64,
    #[serde(deserialize_with = "from_str")]
    pub amount: f64,
}

    let test_data1 = json!({
        "price": "1.2345",
        "amount": "2.1234"
    });

    let test_data2 = json!({
        "price": "-0.345",
        "amount": "-.1234"
    });

    let test_data3 = json!({
        "price": "0",
        "amount": "-1"
    });

    let test_data4 = json!({
        "price": "1.",
        "amount": "2"
    });

    let expected = TestQuote {
        price: 1.2345,
        amount: 2.1234,
    };

    let actual: Result<TestQuote, serde_json::Error> = serde_json::from_str(
        test_data1.to_string().as_str()
    );
    assert_eq!(
        expected,
        actual.unwrap()
    );

    let expected = TestQuote {
        price: -0.345,
        amount: -0.1234,
    };
    let actual: Result<TestQuote, serde_json::Error> = serde_json::from_str(
        test_data2.to_string().as_str()
    );
    assert_eq!(
        expected,
        actual.unwrap()
    );

    let expected = TestQuote {
        price: 0.0,
        amount: -1.0,
    };
    let actual: Result<TestQuote, serde_json::Error> = serde_json::from_str(
        test_data3.to_string().as_str()
    );
    assert_eq!(
        expected,
        actual.unwrap()
    );

    let expected = TestQuote {
        price: 1.0,
        amount: 2.0,
    };
    let actual: Result<TestQuote, serde_json::Error> = serde_json::from_str(
        test_data4.to_string().as_str()
    );
    assert_eq!(
        expected,
        actual.unwrap()
    );
}

#[test]
fn test_deserialize_from_str_fails() {
    use serde_json::json;

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    pub struct TestQuote {
    #[serde(deserialize_with = "from_str")]
    pub price: f64,
    #[serde(deserialize_with = "from_str")]
    pub amount: f64,
}

    let test_data1 = json!({
        "price": "1.2345",
        "amount": "--2.1234"
    });

    let test_data2 = json!({
        "price": "",
        "amount": "2.1234"
    });

    let test_data3 = json!({
        "price": "invalid",
        "amount": "2.1234"
    });

    let test_data4 = json!({
        "xprice": "1.2345",
        "xamount": "2.1234"
    });

    let actual: Result<TestQuote, serde_json::Error> = serde_json::from_str(
        test_data1.to_string().as_str()
    );
    assert!(actual.is_err());

    let actual: Result<TestQuote, serde_json::Error> = serde_json::from_str(
        test_data2.to_string().as_str()
    );
    assert!(actual.is_err());

    let actual: Result<TestQuote, serde_json::Error> = serde_json::from_str(
        test_data3.to_string().as_str()
    );
    assert!(actual.is_err());

    let actual: Result<TestQuote, serde_json::Error> = serde_json::from_str(
        test_data4.to_string().as_str()
    );
    assert!(actual.is_err());
}