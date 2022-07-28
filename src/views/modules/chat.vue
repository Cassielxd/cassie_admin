<template>
  <div class="aui-wrapper ">
    <main class="aui-content">
      <nav class="aui-navbar">
        <div data-tauri-drag-region class="aui-navbar__body">
          <el-menu class="aui-navbar__menu mr-auto" mode="horizontal">
          </el-menu>
          <el-menu class="aui-navbar__menu" mode="horizontal">
            <el-menu-item index="2" @click="minScreenHandle()">
              <svg class="icon-svg aui-navbar__icon-menu" aria-hidden="true">
                <use xlink:href="#icon-minus"></use>
              </svg>
            </el-menu-item>
            <el-menu-item index="4" @click="closeWindow()">
              <svg class="icon-svg aui-navbar__icon-menu" aria-hidden="true">
                <use xlink:href="#icon-close"></use>
              </svg>
            </el-menu-item>
          </el-menu>
        </div>
      </nav>
      <JwChat-index :config="config" :showRightBox='true' scrollType="scroll" :taleList="taleList" @enter="bindEnter"
        @clickTalk="talkEvent" width="770" height="540" v-model="inputMsg" :toolConfig="tool">
        <JwChat-rightbox :config="rightConfig" @click="rightClick" />
      </JwChat-index>
    </main>

  </div>

</template>
<script>
import Cookies from 'js-cookie'
import { appWindow } from '@tauri-apps/api/window'
export default {
  data() {
    return {
      websock: null,
      inputMsg: '',
      taleList: [],
      tool: {
        show: ['file', 'history', 'img', ['文件1', '', '美图']],
        // showEmoji: false,
        callback: this.toolEvent
      },
      config: {
        img: 'https://portrait.gitee.com/uploads/avatars/user/643/1930682_stringlxd_1643358838.png!avatar200',
        name: 'Cassie axum',
        dept: '最简单、最便捷 最牛皮的 rust框架案例',
        callback: this.bindCover,
        historyConfig: {
          show: true,
          tip: '加载更多',
          callback: this.bindLoadHistory,
        },
        quickList: [
          { text: ' 这里是快捷方式' },
        ]
      },
      rightConfig: {
        listTip: '当前在线',
        notice: '【公告】这是cassie axum开源项目，供rust爱好者学习扩展思维',
        list: []
      }
    }
  },
  mounted() {
    this.initWebSocket();
  },
  methods: {
    closeWindow() {
      this.$confirm("您是否要退出当前程序", "关闭窗口", {
        confirmButtonText: "关闭",
        cancelButtonText: "取消",
        type: 'warning'
      }).then(() => {
        this.wsClose();
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
    rightClick(type) {
      console.log('rigth', type)
    },
    talkEvent(e) {
      console.log('t', e)
    },
    bindEnter() {
      //群消息发送
      const msg = this.inputMsg
      if (!msg) return;
      const msgObj = {
        "date": "2020/05/20 23:19:07",
        "text": { "text": msg },
        "mine": true,
        "name": "Cassie",
        "img": "https://portrait.gitee.com/uploads/avatars/user/643/1930682_stringlxd_1643358838.png!avatar200"
      }
      this.taleList.push(msgObj);
      //如果不是群消息 需要发给固定的某人 需要 填 from   to
      this.websocketSend(JSON.stringify({ 'mt': 'Msg', body: { text: msg, from: "", to: "" } }));
    },
    /**
     * @description: 
     * @param {*} type 当前点击的按钮
     * @param {*} plyload 附加文件或者需要处理的数据
     * @return {*}
     */
    toolEvent(type, plyload) {
      console.log('tools', type, plyload)
    },
    /**
    * @description: 点击加载更多的回调函数
    * @param {*}
    * @return {*}
    */
    bindLoadHistory() {
      const history = new Array(3).fill().map((i, j) => {
        return {
          "date": "现在",
          "text": { "text": j + new Date() },
          "mine": false,
          "name": "JwChat",
          "img": "image/three.jpeg"
        }
      })
      let list = history.concat(this.list)
      this.list = list
      console.log('加载历史', list, history)
    },
    bindCover(type) {
      console.log('header', type)
    },
    bindWinBar(play = {}) {
      const { type, data = {} } = play
      console.log(data);
      if (type === 'winBar') {
        const { id, dept, name, img } = data
        this.winBarConfig.active = id
      }
    },
    //-----------------------------------------------------
    initWebSocket() {
      let token = Cookies.get('access_token') || '';
      const wsuri =`${window.SITE_CONFIG['wsURL']}?access_token=` + token;
      this.websock = new WebSocket(wsuri);
      this.websock.onmessage = this.websocketOnMessage;
      this.websock.onopen = this.websocketOnOpen;
      this.websock.onerror = this.websocketOnError;
      this.websock.onclose = this.websocketClose;
    },
    websocketOnOpen(event) {
      var that = this;
      //立即获取在线用户列表
      that.websocketSend(JSON.stringify({ 'mt': 'UserList' }));
      setInterval(function () {
        //定时心跳防止连接断开
        that.websocketSend(JSON.stringify({ 'mt': 'Ping' }));
      }, 15000);
      setInterval(function () {
        //定时获取在线列表 可能会有上线用户 下线用户
        that.websocketSend(JSON.stringify({ 'mt': 'UserList' }));
      }, 5000);
    },
    wsClose() {
      this.websock.close();
    },
    websocketOnError(e) {
      this.initWebSocket();
    },
    websocketOnMessage(e) {
      const redata = JSON.parse(e.data);
      this.handleMsg(redata);
    },
    websocketSend(data) {
      this.websock.send(data);
    },
    websocketClose(e) {
      console.log('断开连接', e);
    },
    handleMsg(redata) {
      switch (redata.mt) {
        case "Msg": {
          //消息接收
          let mesg = {
            "date": redata.date||'',
            "text": { "text": redata.body.text },
            "mine": false,
            "name": redata.body.from,
            "img": "https://portrait.gitee.com/uploads/avatars/user/643/1930682_stringlxd_1643358838.png!avatar200"
          };
          this.taleList.push(mesg);
          break;
        }
        case "Ping": {
          //心跳
          console.log("心跳回执:",redata.body.text);
          break;
        }
        case "UserList": {
          //获取在线用户
          const userData = JSON.parse(redata.body.text);
          this.getUserList(userData);
          break;
        }
      }
    },
    getUserList(userData) {
      let list = [];
      if (userData) {
        for (let i = 0; i < userData.length; i++) {
          list.push({
            name: userData[i].username,
            "img": "https://portrait.gitee.com/uploads/avatars/user/643/1930682_stringlxd_1643358838.png!avatar200"
          });
        }
      }
      this.rightConfig.list = list;
    }

  }
}
</script>

<style>
</style>