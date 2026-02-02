module.exports = {
    rules: {
        'vue/html-self-closing': ['error', {
            html: {
                void: 'always',
                normal: 'never',
                component: 'always'
            },
            svg: 'always',
            math: 'always'
        }]
    }
}