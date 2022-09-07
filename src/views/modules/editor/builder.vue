<template>
  <el-container class="aui-content">
    <el-aside>
      <el-tabs type="card" v-model="siderType">
        <el-tab-pane label="组件" name="widget">
          <!-- 组件列表 -->
          <widget-list
              :list="widgetList"
              @onWidgetMouseDown="onWidgetMouseDown"
          />
        </el-tab-pane>
        <el-tab-pane label="图层" name="layer">
          <!-- 图层列表 -->
          <sortable v-model="list" @change="onSortChange">
            <transition-group>
              <el-tag
                  class="layer"
                  style="border-radius: 0px"
                  v-for="item in list"
                  :key="item.id"
                  type="info"
                  effect="plain"
                  size="medium"
              >
                {{ item.label }}
              </el-tag>
            </transition-group>
          </sortable>
        </el-tab-pane>

      </el-tabs>

    </el-aside>
    <el-container>
      <el-header>
        <el-button-group>
          <el-button v-for="(value,type) in paperTypes" :type="curPaperType === type ? 'primary' : ' '"
                     @click="setPaper(type,value)" :key="type">
            {{ type }}
          </el-button>
        </el-button-group>
      </el-header>

          <div
              :style="[baseStyles,overridingStyles]"
              ref="panel"
              @dragover.prevent
              @drop="onDrop"
              @mousedown="onPanelMouseDown"
          >

            <vdr

                v-for="(item, i) in list"
                :x="item.x"
                :y="item.y"
                :z="item.z"
                :w="item.w"
                :h="item.h"
                :debug="false"
                :active="item.focused"
                :handles="item.handles"
                :snap="true"
                :snapTolerance="1"
                ref="widget"
                :parent="true"
                @refLineParams="getRefLineParams"
                @contextmenu.native.prevent="onContextMenuOpen($event, item)"
                @dragging="(x, y)=>onDragStartCallback(x, y,item)"
                @resizing="(x, y, width, height)=>onResize(x, y, width, height,item)"
                @mousedown.native.stop="onFocus(item)"
            >
              <component
                  class="inner-widget"
                  :is="item.component"
                  :value="item.value"
                  :styles="item.styles"
                  @drop.native.stop="onDrop($event, i)"
              />

            </vdr>
            <!--辅助线-->
            <span class="ref-line v-line"
                  v-for="item in vLine"
                  :key="item.id"
                  v-show="item.display"
                  :style="{ left: item.position, top: item.origin, height: item.lineLength}"
            />
            <span class="ref-line h-line"
                  v-for="item in hLine"
                  :key="item.id"
                  v-show="item.display"
                  :style="{ top: item.position, left: item.origin, width: item.lineLength}"
            />
            <!--辅助线END-->
          </div>
      <context-menu ref="contextMenu">
        <li>
          <a href="#" @click.prevent="onLayerTop">置顶</a>
        </li>
        <li>
          <a href="#" @click.prevent="onLayerBottom">置底</a>
        </li>
        <li>
          <a href="#" @click.prevent="onLayerRemove">删除</a>
        </li>
      </context-menu>
    </el-container>
    <el-aside width="400px">
      <style-sider
          :current="current"
          :form="currentForm"
          @change="onStyleChange"
      />
    </el-aside>
  </el-container>
</template>

<script>
// 小组件列表
import WidgetList from './components/widget-list'
// 样式配置区域
import StyleSider from './components/style-sider'
// 左侧小组件
import CustomText from './components/custom-text'
import CustomVideo from './components/custom-video'
import CustomImage from './components/custom-image'
import CustomVline from './components/custom-vline'
import CustomTable from './components/custom-table'
// 右键菜单
import ContextMenu from 'vue-context'

// 静态配置
import * as CONFIG from './constants/config'
import { cloneDeep } from 'lodash'
import CustomHline from './components/custom-hline'

let currentId = 1
let widgetX = 0
let widgetY = 0
let currentWidget = null

export default {
  components: {
    WidgetList,
    CustomText,
    CustomVideo,
    CustomImage,
    ContextMenu,
    StyleSider,
    CustomHline,
    CustomVline,
    CustomTable
  },
  data () {
    return {
      divStyles: {
        width: '215mm',
        height: '296.6mm'
      },
      baseStyles: {
        width: '210mm',
        height: '296.6mm'
      },
      overridingStyles: {
        border: '1px solid #917a7a',
        position: 'relative',
        margin: '0 auto'
      },
      presetLine: [{ type: 'l', site: 100 }, { type: 'v', site: 200 }],
      vLine: [],
      hLine: [],
      siderType: 'widget',
      list: [],
      widgetList: CONFIG.WIDGET_LIST,
      // 是否正在框选
      isFraming: false,
      // 是否可以多选
      multiable: false,
      curPaperType: 'A4',
      curPaper: {
        type: 'A4',
        width: 210,
        height: 296.6
      },
      paperTypes: {
        'A3': {
          width: 420,
          height: 296.6
        },
        'A4': {
          width: 210,
          height: 296.6
        },
        'A5': {
          width: 210,
          height: 147.6
        },
        'B3': {
          width: 500,
          height: 352.6
        },
        'B4': {
          width: 250,
          height: 352.6
        },
        'B5': {
          width: 250,
          height: 175.6
        }
      },
    }
  },
  computed: {
    current () {
      return this.list.find(item => item.focused)
    },
    currentForm () {
      if (!this.current) {
        return {}
      }
      return CONFIG.WIDGET_LIST.find(item => this.current.type === item.type)
    },
  },
  methods: {
    setPaper (type, value) {
      try {
        if (Object.keys(this.paperTypes).includes(type)) {
          this.curPaper = { type: type, width: value.width, height: value.height }
          this.baseStyles = {
            width: value.width + 'mm',
            height: value.height + 'mm',
          }
          this.divStyles = {
            width: value.width + 5 + 'mm',
            height: value.height + 5 + 'mm',
          }
          this.curPaperType = type
        } else {
          this.curPaper = { type: 'other', width: value.width, height: value.height }
        }
        this.$nextTick(() => {
          window.dispatchEvent(new Event('resize'))
        })
      } catch (error) {
        this.$message.error(`操作失败: ${error}`)
      }
    },
    onResize: function (x, y, width, height, item) {
      item.x = x
      item.y = y
      item.w = width
      item.h = height
    },
    onDragStartCallback (x, y, item) {
      item.x = x
      item.y = y
    },
    getRefLineParams (params) {

      const { vLine, hLine } = params
      let id = 0
      this.vLine = vLine.map(item => {
        item['id'] = ++id
        return item
      })
      this.hLine = hLine.map(item => {
        item['id'] = ++id
        return item
      })
    },

    onPanelMouseDown (e) {
      this.list.forEach((item) => {
        item.focused = false
      })
    },
    // 当图层样式改变时
    onStyleChange (id, newStyles) {
      this.list = this.list.map((item) => {
        if (item.id === id) {
          item.style = newStyles
        }
        return item
      })
    },
    // 保存
    async save () {

    },
    onSortChange () {
      const len = this.list.length
      this.list.forEach((item, i) => {
        item.z = len - i
      })
    },
    // 给list排序
    sortList () {
      this.list.sort((a, b) => b.z - a.z)
    },
    findTopLayerZ (currentItem) {
      const maxZ = Math.max(...this.list.map(item => item.z)) || 0
      if (currentItem.z === maxZ) {
        alert('已经是最顶层了')
        return
      }
      return maxZ
    },
    findBottomLayerZ (currentItem) {
      const minZ = Math.min(...this.list.map(item => item.z)) || 0
      if (currentItem.z === minZ) {
        alert('已经是最底层了')
        return false
      }
      return minZ
    },
    onLayerBottom () {
      const currentItem = this.list.find(item => item.focused)
      const minZ = this.findBottomLayerZ(currentItem)
      if (minZ === false) {
        return
      }
      if (minZ - 1 < 0) {
        this.list = this.list.map(item => {
          item.z -= minZ - 1
          return item
        })
        currentItem.z = 0
      } else {
        currentItem.z = minZ - 1
      }
      this.sortList()
    },
    onLayerTop () {
      const currentItem = this.list.find(item => item.focused)
      const maxZ = this.findTopLayerZ(currentItem)
      if (!maxZ) {
        return
      }
      currentItem.z = maxZ + 1
      this.sortList()
    },
    // 移除图层
    onLayerRemove () {
      this.list = this.list.filter(item => !item.focused)
      this.sortList()
    },
    // 让当前项获取焦点 其他项失去焦点
    onFocus (currentItem) {
      this.list = this.list.map(item => {
        item.focused = item.id === currentItem.id
        return item
      })
    },
    // 右键菜单打开事件
    onContextMenuOpen (e, item) {
      // 打开右键菜单
      this.$refs.contextMenu.open(e)
      // 给当前项获取焦点
      this.onFocus(item)
    },
    // 放置
    onDrop (e, i) {
      let x = e.offsetX - widgetX
      let y = e.offsetY - widgetY
      // 放置在其他图层上时
      if (i !== undefined) {
        x += this.list[i].x
        y += this.list[i].y
      }
      // 关闭右键菜单
      this.$refs.contextMenu.close()
      // 新增面板项
      const newItem = {
        id: currentId++,
        //
        x,
        y,
        // 新增的面板项层级应该最高
        z: !this.list.length
            ? 0
            : Math.max(...this.list.map(item => item.z)) + 1,
        ...currentWidget.default, // 生成默认的宽高数据 w, h, value
        label: currentWidget.label,
        component: currentWidget.component, // 新增的组件名
        type: currentWidget.type, // 新增组件的类型
        styles: cloneDeep(currentWidget.styles), // 新增组件的类型
        handles:currentWidget.handles
      }

      this.list.push(newItem)
      this.onFocus(newItem)
      this.sortList()
    },
    // 在小组件鼠标落下的时候
    onWidgetMouseDown (e, widget) {
      widgetX = e.offsetX
      widgetY = e.offsetY
      currentWidget = widget
    },
    // 通过type找宽高信息
    findDefaultWithType (type) {
      return CONFIG.WIDGET_LIST.find(item => item.type === type).default
    },
  },
  mounted () {
  },

  beforeDestroy () {
  },
}
</script>

<style>
.el-header, .el-footer {
  background-color: #FFFFFF;
  color: #333;
  text-align: center;
  line-height: 60px;
}

.el-aside {
  color: #333;
}

.el-main {
  background-color: #FFFFFF;
  color: #333;
  text-align: center;
}

body > .el-container {
  height: 100%;
}

.el-container:nth-child(5) .el-aside,
.el-container:nth-child(6) .el-aside {
  line-height: 260px;
}

.el-container:nth-child(7) .el-aside {
  line-height: 320px;
}

.inner-widget {
  height: 100%;
  width: 100%;
}

.layer {
  width: 100%;
  height: 50px;
}

.layer:hover {
  background: #e9e9e9;
}
</style>
