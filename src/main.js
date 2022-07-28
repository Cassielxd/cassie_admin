import Vue from 'vue'
import Element from 'element-ui'
import App from '@/App'
import i18n from '@/i18n'
import router from '@/router'
import store from '@/store'
import '@/icons'
import '@/element-ui/theme/index.css'
import '@/assets/scss/aui.scss'
import http from '@/utils/request'
import renRadioGroup from '@/components/ren-radio-group'
import renSelect from '@/components/ren-select'
import renProcessRunning from '@/components/ren-process-running'
import renProcessDetail from '@/components/ren-process-detail'
import renDeptTree from '@/components/ren-dept-tree'
import renRegionTree from '@/components/ren-region-tree'
import { hasPermission, getDictLabel } from '@/utils'
import cloneDeep from 'lodash/cloneDeep'
import JwChat from 'jwchat'
Vue.use(JwChat)

import Tinymce from '@/components/tinymce/index.vue'
Vue.component('tinymce', Tinymce)

import '@/utils/form-generator/styles/index.scss'
import SvgIcon from '@/utils/form-generator/components/SvgIcon'
Vue.component('svg-icon', SvgIcon)

import 'xe-utils'
import VXETable from 'vxe-table'
import 'vxe-table/lib/index.css'
Vue.use(VXETable)

Vue.config.productionTip = false

Vue.use(Element, {
  size: 'default',
  i18n: (key, value) => i18n.t(key, value)
})

Vue.use(renRadioGroup)
Vue.use(renSelect)
Vue.use(renDeptTree)
Vue.use(renRegionTree)
Vue.use(renProcessRunning)
Vue.use(renProcessDetail)

// 挂载全局
Vue.prototype.$http = http
Vue.prototype.$hasPermission = hasPermission
Vue.prototype.$getDictLabel = getDictLabel

// 保存整站vuex本地储存初始状态
window.SITE_CONFIG['storeState'] = cloneDeep(store.state)

new Vue({
  i18n,
  router,
  store,
  render: h => h(App)
}).$mount('#app')
