module.exports = {
    chainWebpack: config => {
        config.module
            .rule('vue')
            .use('vue-loader')
            .tap(options => {
                return options;
            });
    },
    css: {
        sourceMap: true
    },
    // assetsDir: '../../assets/',
    indexPath: 'index.tpl'
    // outputDir: '../main/resources/',
}