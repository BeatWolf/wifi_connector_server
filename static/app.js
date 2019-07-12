var app = new Vue({
    el: '#app',
    data: {
        message: 'Hello Vue!'
    },
    mounted() {
        axios.get('api')
            .then(response => (this.message = response.data))
    }
})