var app = new Vue({
    el: '#app',
    data: {
        wifis: []
    },
    mounted() {
        axios.get('api/wifis')
            .then(response => (this.wifis = response.data))
    }
})