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
    open: false,
    port: 8001,
    overlay: {
      errors: false,
      warnings: false
    },
    proxy: {
      // 代理
      "/luck": {
        target: "http://localhost:3000/",     //要代理访问的路径
        ws: false,// 是否启用websockets
        changeOrigin: true,//开启代理：在本地会创建一个虚拟服务端，然后发送请求的数据，并同时接收请求的数据，这样服务端和服务端进行数据的交互就不会有跨域问题
        pathRewrite: {
          "^/luck": ""//这里理解成用'/api'代替target里面的地址,比如我要调用'http://192.168.0.45:8088/user/getuserlist'，直接写'/api/user/getuserlist'即可
        }
      }
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
