import axios from "axios";

const ipaddr = 'http://127.0.0.1:3000';

axios({
    baseURL: ipaddr,
    url: "/match/latest",
    method: "get",
    params: {
        account_id: 417817047
    }
})
    .then(async resp => {
        console.log(resp.data)
    })
    .catch(e => {
        console.error(e)
    })


