# rust_hello

## "src/rust_hello"

    restful api mock service

    dfx deploy rust_hello --network=ic --with-cycles 200000000000 --wallet tfsfh-haaaa-aaaah-qagdq-cai


## "src/rust_hello_dioxus"
    
    dioxus front service
    run.sh



## mock 接口 
restful api 同时支持 Get/Post 请求
1. {{canister_id_pro}}/v1/api/dt_market_source

   豆瓣好物接口

2. {{canister_id_pro}}/v1/api/login

   登陆接口 

    {
    "username": "manue1",
    "password": "xxx"
    }

3. {{canister_id_pro}}/v1/api/greet

    echo 接口