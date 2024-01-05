pub struct AddressType {
    pub(crate) name: &'static str,
    pub(crate) min_rank: i32,
    pub(crate) max_rank: i32,
}

const HOUSE: AddressType = AddressType {
    name: "house",
    min_rank: 29,
    max_rank: 30,
};
const STREET: AddressType = AddressType {
    name: "street",
    min_rank: 26,
    max_rank: 28,
};
const LOCALITY: AddressType = AddressType {
    name: "locality",
    min_rank: 22,
    max_rank: 25,
};
const DISTRICT: AddressType = AddressType {
    name: "district",
    min_rank: 17,
    max_rank: 21,
};
const CITY: AddressType = AddressType {
    name: "city",
    min_rank: 13,
    max_rank: 16,
};
const COUNTY: AddressType = AddressType {
    name: "county",
    min_rank: 10,
    max_rank: 12,
};
const STATE: AddressType = AddressType {
    name: "state",
    min_rank: 5,
    max_rank: 9,
};
const COUNTRY: AddressType = AddressType {
    name: "country",
    min_rank: 4,
    max_rank: 4,
};

pub fn address_types() -> Vec<AddressType> {
    return vec![
        HOUSE, STREET, LOCALITY, DISTRICT, CITY, COUNTY, STATE, COUNTRY,
    ];
}

pub fn from_rank(rank: i32) -> Option<AddressType> {
    let address_types = address_types();
    for a in address_types {
        if rank >= a.min_rank && rank <= a.max_rank {
            return Some(a);
        }
    }
    return None;
}