/**
 * 配置参考: https://cli.vuejs.org/zh/config/
 */
const MonacoEditorPlugin = require('monaco-editor-webpack-plugin')

const path = require('path')
function resolve (dir) {
  return path.join(__dirname, dir)
}
module.exports = {
  publicPath: process.env.NODE_ENV === 'production' ? './' : '/',
  chainWebpack: config => {
    config.module
        .rule('svg')
        .exclude.add(resolve('src/icons'))
        .end()
    config.module
        .rule('icons')
        .test(/\.svg$/)
        .include.add(resolve('src/icons'))
        .end()
        .use('svg-sprite-loader')
        .loader('svg-sprite-loader')
        .options({
          symbolId: 'icon-[name]'
        })
        .end()
  },
  // 默认打开eslint效验，如果需要关闭，设置成false即可
  lintOnSave: false,
  productionSourceMap: false,
  devServer: {
    open: true,
    port: 8001,
    overlay: {
      errors: false,
      warnings: false
    }
  },
  configureWebpack: {
    plugins: [
      new MonacoEditorPlugin({
        // https://github.com/Microsoft/monaco-editor-webpack-plugin#options
        languages: ['javascript', 'typescript', 'sql', 'java']
      })
    ]
  }
}
