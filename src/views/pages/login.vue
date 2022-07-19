<template>
  <div class="aui-wrapper aui-page__login">
    <div class="aui-content__wrapper">
      <main class="aui-content">
        <div class="login-header">
          <h2 class="login-brand">{{ $t('brand.lg') }}</h2>
        </div>
        <div class="login-body">
          <h3 class="login-title">{{ $t('login.title') }}</h3>
          <el-form :model="dataForm" :rules="dataRule" ref="dataForm" @keyup.enter.native="dataFormSubmitHandle()" status-icon>
            <el-form-item>
              <el-select v-model="$i18n.locale" class="w-percent-100">
                <el-option v-for="(val, key) in i18nMessages" :key="key" :label="val._lang" :value="key"></el-option>
              </el-select>
            </el-form-item>
            <el-form-item prop="username">
              <el-input v-model="dataForm.username" :placeholder="$t('login.username')">
                <span slot="prefix" class="el-input__icon">
                  <svg class="icon-svg" aria-hidden="true"><use xlink:href="#icon-user"></use></svg>
                </span>
              </el-input>
            </el-form-item>
            <el-form-item prop="password">
              <el-input v-model="dataForm.password" type="password" :placeholder="$t('login.password')">
                <span slot="prefix" class="el-input__icon">
                  <svg class="icon-svg" aria-hidden="true"><use xlink:href="#icon-lock"></use></svg>
                </span>
              </el-input>
            </el-form-item>
            <el-form-item prop="captcha">
              <el-row :gutter="20">
                <el-col :span="14">
                  <el-input v-model="dataForm.vcode" :placeholder="$t('login.captcha')">
                    <span slot="prefix" class="el-input__icon">
                      <svg class="icon-svg" aria-hidden="true"><use xlink:href="#icon-safetycertificate"></use></svg>
                    </span>
                  </el-input>
                </el-col>
                <el-col :span="10" class="login-captcha">
                  <img :src="'data:image/jpeg;base64,'+captchaPath" @click="getCaptcha()">
                </el-col>
              </el-row>
            </el-form-item>
            <el-form-item>
              <el-button type="primary" @click="dataFormSubmitHandle()" class="w-percent-100">{{ $t('login.title') }}</el-button>
            </el-form-item>
          </el-form>
        </div>
        <div class="login-footer">
          <p>
            <a href="https://gitee.com/stringlxd/cassie_axum" target="_blank">gitee</a>
          </p>
         
        </div>
      </main>
    </div>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api'
import Cookies from 'js-cookie'
import debounce from 'lodash/debounce'
import { messages } from '@/i18n'
import { getUUID } from '@/utils'
export default {
  data () {
    return {
      i18nMessages: messages,
      captchaPath: '',
      dataForm: {
        username: '',
        password: '',
        uuid: '',
        vcode: ''
      }
    }
  },
  computed: {
    dataRule () {
      return {
        username: [
          { required: true, message: this.$t('validate.required'), trigger: 'blur' }
        ],
        password: [
          { required: true, message: this.$t('validate.required'), trigger: 'blur' }
        ],
        vcode: [
          { required: true, message: this.$t('validate.required'), trigger: 'blur' }
        ]
      }
    }
  },
  async created () {

   await this.getCaptcha();
    if(window.__TAURI__){
      await invoke('plugin:sqlite|del',{key:"access_token"});
    }
    
   
  },
  methods: {
     set_token(token){
      
     return  invoke('plugin:sqlite|save',{key:"access_token",value:token});
    },
    // 获取验证码
    async getCaptcha () {
      this.dataForm.uuid = getUUID()
      let e =await this.$http.get(`/captcha/${this.dataForm.uuid}`);
      this.captchaPath=e.data.data;
    },
     
    // 表单提交
     dataFormSubmitHandle: debounce(  function () {
      this.$refs['dataForm'].validate((valid) => {
        if (!valid) {
          return false
        }
        this.$http.post('/login', this.dataForm
        ).then(({ data: res }) => {
          if (res.code != 0) {
            this.getCaptcha()
            return this.$message.error(res.msg)
          }
          Cookies.set('access_token', res.data.access_token)
          if(window.__TAURI__){
            this.set_token(res.data.access_token);
          }
          
          this.$router.replace({ name: 'home' })
        }).catch(() => {})
      })
    }, 1000, { 'leading': true, 'trailing': false })
  }
}
</script>
