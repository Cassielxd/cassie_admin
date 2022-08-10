import axios from 'axios'
import Cookies from 'js-cookie'
import router from '@/router'
import qs from 'qs'
import { clearLoginInfo } from '@/utils'
import isPlainObject from 'lodash/isPlainObject'
import { invoke } from '@tauri-apps/api'
import axiosTauriAdapter from 'axios-tauri-adapter';

const http = axios.create({
  baseURL: window.SITE_CONFIG['apiURL'],
  adapter: axiosTauriAdapter,
  timeout: 1000 * 180
})

function  get_token (){
  return invoke('plugin:sqlite|get',{key:"access_token"});
 }


/**
 * 请求拦截
 */
http.interceptors.request.use(async config => {
  config.headers['Accept-Language'] = Cookies.get('language') || 'zh-CN'
  if (config.url !== '/login') {
    let t = Cookies.get('access_token') || '';
    if(t==''&& window.__TAURI__){
      let data =await get_token();
      t =data[0];
      Cookies.set('access_token', data[0]);
    }
    config.headers['access_token'] = t;
  }
  // 默认参数
  var defaults = {}
  // 防止缓存，GET请求默认带_t参数
  if (config.method === 'get') {
    config.params = {
      ...config.params,
      ...{ '_t': new Date().getTime() }
    }
  }
  if (isPlainObject(config.params)) {
    config.params = {
      ...defaults,
      ...config.params
    }
  }
  if (isPlainObject(config.data)) {
    config.data = {
      ...defaults,
      ...config.data
    }
    if (/^application\/x-www-form-urlencoded/.test(config.headers['content-type'])) {
      config.data = qs.stringify(config.data)
    }
  }

  // get序列化
  config.paramsSerializer = function (params) {
    return qs.stringify(params, { arrayFormat: 'repeat' })
  }

  return config
}, error => {
  return Promise.reject(error)
})

/**
 * 响应拦截
 */
http.interceptors.response.use(response => {
  if (response.data.code === 401 || response.data.code === 10001) {
    clearLoginInfo()
    router.replace({ name: 'login' })
    return Promise.reject(response.data.msg)
  }
  return response
}, error => {
  console.error(error)
  return Promise.reject(error)
})

export default http
