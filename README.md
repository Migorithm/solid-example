# Mid-mile challenge

This is a mock example to show how SOLID is applied to Rust web developement


## Requirement



## How to run
```sh
cargo run
```


## API spec
`http://localhost/device_groups`
- device group registration API
    - POST
    - BODY : {"deviceGroupSerial": String }


`http://localhost/devices`
- device registration API
    - POST 
    - BODY : {"serialNumber": String, "deviceGroupSerial" : String}

- temperature saving API
    - PATCH
    - BODY :  {"serialNumber": String, "interval" : Number, "temperatures": String, "registered_at": String}


`/devices/temperature`
- device average temperature 
    - GET
    - QUERY PARAMS
        - serialNumber: String
        - startDate : String
        - endDate: String

`/device_groups/temperature`
- device group average temperatures
    - GET
    - QUERY PARAMS: 
        - deviceGroupSerial : String
        - startDate : String
        - endDate: String



## Test
Each test is designed so you can run without worries about concurrency
```sh
cargo test
```


## ERD
Please refer to domain and its `mod`'s to see how entity relationships are drawn.


## Database
In memory, `MockDb` is implemented for the sake of simplicity, which assumes locking mechanism with the use of signleton pattern with Arc, RwLock. 


