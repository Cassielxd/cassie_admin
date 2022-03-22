<template>
  <el-submenu v-if="group.children && group.children.length >= 1" :index="group.group_code"
              :popper-append-to-body="false">
    <template slot="title">
      <span>{{ group.name }}</span>
    </template>
    <sub-group v-for="group in group.children" @selectGroup="selectGroup" :key="group.id" :group="group"></sub-group>
  </el-submenu>
  <el-menu-item v-else :index="group.group_code" ref="li">
    <a href="javascript:;" @click="selectGroup(group.group_code)"><span>{{ group.name }}</span></a>
  </el-menu-item>
</template>

<script>
import SubGroup from './asi-sub-group'

export default {
  name: 'sub-group',
  data () {
    return {
      browserTabOpenList: [
        '42',
      ]
    }
  },
  props: {
    group: {
      type: Object,
      required: true
    }
  },
  components: {
    SubGroup
  },
  created () {
    this.$nextTick(() => {
      if (this.$refs.li) {
        let $li = this.$refs.li.$el
        let $a = $li.firstElementChild
        if ($a) {
          let pl = '0', pr = '0'
          if ($li.currentStyle) {
            pl = $li.currentStyle['paddingLeft']
            pr = $li.currentStyle['paddingRight']
          } else {
            pl = window.document.defaultView.getComputedStyle($li, null)['paddingLeft']
            pr = window.document.defaultView.getComputedStyle($li, null)['paddingRight']
          }
          $li.setAttribute('style', `padding-left: 0; padding-right: 0;`)
          $a.setAttribute('style', `padding-left: ${pl}; padding-right: ${pr};`)
        }
      }
    })

  },
  methods: {
    selectGroup (code) {
      var vm = this
      vm.$emit('selectGroup', code)
    }
  }
}
</script>

<style lang="scss">
.aui-sidebar__menu {
  .el-menu-item > a {
    display: block;
    color: inherit;
    text-decoration: none;
  }
}
</style>
