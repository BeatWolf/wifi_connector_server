var app = new Vue({
    el: '#app',
    data: {
        wifis: [],
        form: {
            parent_id: []
        }
    },
    mounted() {
        axios.get('api/wifis')
            .then(response => (this.wifis = response.data))
    },
    methods: {
        connect(ssid, password) {
            axios.post('api/wifi/'+ssid +"/connect", {password: password})
        }
    }

})