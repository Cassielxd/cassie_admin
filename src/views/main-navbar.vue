<template>
  <nav class="aui-navbar" :class="`aui-navbar--${$store.state.navbarLayoutType}`">
    <div class="aui-navbar__header">
      <h1 class="aui-navbar__brand" @click="$router.push({ name: 'home' })">
        <a class="aui-navbar__brand-lg" href="javascript:;">{{ $t('brand.lg') }}</a>
        <a class="aui-navbar__brand-mini" href="javascript:;">{{ $t('brand.mini') }}</a>
      </h1>
    </div>
    <div data-tauri-drag-region class="aui-navbar__body">
      <el-menu class="aui-navbar__menu mr-auto" mode="horizontal">
        <el-menu-item index="1" @click="$store.state.sidebarFold = !$store.state.sidebarFold">
          <svg class="icon-svg aui-navbar__icon-menu aui-navbar__icon-menu--switch" aria-hidden="true">
            <use xlink:href="#icon-outdent"></use>
          </svg>
        </el-menu-item>
        <el-menu-item index="2" @click="refresh()">
          <svg class="icon-svg aui-navbar__icon-menu aui-navbar__icon-menu--refresh" aria-hidden="true">
            <use xlink:href="#icon-sync"></use>
          </svg>
        </el-menu-item>
        <el-menu-item index="1" class="aui-navbar__avatar">
          <el-dropdown placement="bottom" :show-timeout="0">
            <span class="el-dropdown-link">
              <span>工具</span>
              <i class="el-icon-arrow-down"></i>
            </span>
            <el-dropdown-menu slot="dropdown">
              <el-dropdown-item @click.native="openNew(1)">Js沙箱环境
              </el-dropdown-item>
              <el-dropdown-item @click.native="openNew(2)">源码链接</el-dropdown-item>
              <el-dropdown-item @click.native="openNew(3)">IM</el-dropdown-item>
            </el-dropdown-menu>
          </el-dropdown>
        </el-menu-item>
      </el-menu>
      <el-menu class="aui-navbar__menu" mode="horizontal">
        <el-menu-item index="1" class="aui-navbar__avatar">
          <el-dropdown placement="bottom" :show-timeout="0">
            <span class="el-dropdown-link">
              <img src="http://washcar.cymcar.com/img/avatar.c58e4651.png">
              <span>{{ $store.state.user.realName }}</span>
              <i class="el-icon-arrow-down"></i>
            </span>
            <el-dropdown-menu slot="dropdown">
              <el-dropdown-item @click.native="updatePasswordHandle()">{{ $t('updatePassword.title') }}
              </el-dropdown-item>
              <el-dropdown-item @click.native="logoutHandle()">{{ $t('logout') }}</el-dropdown-item>
            </el-dropdown-menu>
          </el-dropdown>
        </el-menu-item>
        <el-menu-item index="2" @click="minScreenHandle()">
          <svg class="icon-svg aui-navbar__icon-menu" aria-hidden="true">
            <use xlink:href="#icon-minus"></use>
          </svg>
        </el-menu-item>
        <el-menu-item index="3" @click="fullscreenHandle()">
          <svg class="icon-svg aui-navbar__icon-menu" aria-hidden="true">
            <use xlink:href="#icon-border"></use>
          </svg>
        </el-menu-item>
        <el-menu-item index="4" @click="closeWindow()">
          <svg class="icon-svg aui-navbar__icon-menu" aria-hidden="true">
            <use xlink:href="#icon-close"></use>
          </svg>
        </el-menu-item>
      </el-menu>
    </div>
    <!-- 弹窗, 修改密码 -->
    <update-password v-if="updatePasswordVisible" ref="updatePassword"></update-password>
  </nav>
</template>

<script>
import { appWindow, WebviewWindow } from '@tauri-apps/api/window'
import { messages } from '@/i18n'
import UpdatePassword from './main-navbar-update-password'
import { clearLoginInfo } from '@/utils'
export default {
  inject: ['refresh'],
  data() {
    return {
      i18nMessages: messages,
      updatePasswordVisible: false,
      messageTip: false
    }
  },
  components: {
    UpdatePassword
  },
  created() {
  },
  methods: {
    myNoticeRouter() {
      this.$router.replace('sys-notice-user')
    },
    openNew(index) {
      switch (index) {
        case 1: {
          new WebviewWindow('js_play_ground', {
            title: "JsPlayGround",
            url: '/#/jsruntime',
            center: true,
          });
          break;
        }
        case 2: {
          window.open("https://gitee.com/stringlxd");
          break;
        }
        case 3: { 
          new WebviewWindow('chat', {
            resizable:false,
            fullscreen:false,
            title: "cassie chat",
            url: '/#/chat',
            center: true,
          });
          break;
        }
      }

    },
    //关闭窗口
    closeWindow() {
      this.$confirm("您是否要退出当前程序", "关闭窗口", {
        confirmButtonText: "关闭",
        cancelButtonText: "取消",
        type: 'warning'
      }).then(() => {
        appWindow.close();
      })

    },
    //最小化
    minScreenHandle() {
      appWindow.minimize();
    },
    // 全屏
    fullscreenHandle() {
      appWindow.toggleMaximize()
    },
    // 修改密码
    updatePasswordHandle() {
      this.updatePasswordVisible = true
      this.$nextTick(() => {
        this.$refs.updatePassword.init()
      })
    },
    // 退出
    logoutHandle() {
      this.$confirm(this.$t('prompt.info', { 'handle': this.$t('logout') }), this.$t('prompt.title'), {
        confirmButtonText: this.$t('confirm'),
        cancelButtonText: this.$t('cancel'),
        type: 'warning'
      }).then(() => {
        clearLoginInfo()
        this.$router.push({ name: 'login' })
      }).catch(() => { })
    }
  }
}
</script>
