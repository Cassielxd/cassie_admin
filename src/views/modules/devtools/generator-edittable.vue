<template>
  <el-dialog :visible.sync="visible" title="编辑" :close-on-click-modal="false" :close-on-press-escape="false" :fullscreen="true">
    <vxe-table
      border
      row-key
      ref="xTable"
      class="sortable-row-gen"
      size="mini"
      :data="tableData"
      :checkbox-config="{checkStrictly: true}"
      :edit-config="{trigger: 'click', mode: 'cell', icon: 'fa fa-pencil'}">
      <vxe-table-column type="seq" width="35"></vxe-table-column>
      <vxe-table-column width="34" title="拖动">
        <template v-slot>
          <span class="drag-btn">
            <i class="vxe-icon--menu"></i>
          </span>
        </template>
        <template v-slot:header>
          <el-tooltip class="item" effect="dark" content="按住后可以上下拖动排序" placement="top-start">
            <i class="vxe-icon--question"></i>
          </el-tooltip>
        </template>
      </vxe-table-column>
      <vxe-table-column title="字段" align="center">
        <vxe-table-column field="columnName" width="140" title="字段名"></vxe-table-column>
        <vxe-table-column field="attrName" width="140" title="属性名" :edit-render="{name: 'input', type: 'visible'}"></vxe-table-column>
        <vxe-table-column field="columnType" width="140" title="字段类型"></vxe-table-column>
        <vxe-table-column field="attrType" width="140" title="属性类型" :edit-render="{name: 'select', type: 'visible', options: typeList }"></vxe-table-column>
        <vxe-table-column field="columnComment" width="140" title="字段说明" :edit-render="{name: 'input', type: 'visible'}"></vxe-table-column>
      </vxe-table-column>
      <vxe-table-column title="列表" align="center">
        <vxe-table-column field="list" width="75" title="列表">
          <template v-slot="{ row }">
            <vxe-checkbox v-model="row.list"></vxe-checkbox>
          </template>
        </vxe-table-column>
        <vxe-table-column field="query" width="75" title="查询">
          <template v-slot="{ row }">
            <vxe-checkbox v-model="row.query"></vxe-checkbox>
          </template>
        </vxe-table-column>
        <vxe-table-column field="queryType" width="140" title="查询方式" :edit-render="{name: 'select', type: 'visible', options: queryList }"></vxe-table-column>
      </vxe-table-column>
      <vxe-table-column title="表单" align="center">
        <vxe-table-column field="form" width="70" title="表单">
          <template v-slot="{ row }">
            <vxe-checkbox v-model="row.form"></vxe-checkbox>
          </template>
        </vxe-table-column>
        <vxe-table-column field="required" width="70" title="必填">
          <template v-slot="{ row }">
            <vxe-checkbox v-model="row.required"></vxe-checkbox>
          </template>
        </vxe-table-column>
        <vxe-table-column field="formType" width="140" title="表单类型" :edit-render="{name: 'select', type: 'visible', options: formTypeList }"></vxe-table-column>
        <vxe-table-column field="dictName" width="140" title="字典名称" :edit-render="{name: 'select', type: 'visible', options: dictList }"></vxe-table-column>
      </vxe-table-column>
    </vxe-table>
    <template slot="footer">
      <el-button @click="visible = false">取消</el-button>
      <el-button type="primary" @click="updateHandle()">保存</el-button>
    </template>
  </el-dialog>
</template>
<script>
import debounce from 'lodash/debounce'
import Sortable from 'sortablejs'
export default {
  data () {
    return {
      visible: false,
      loading: false,
      typeList: [],
      dictList: [],
      queryList: [
        { label: '=', value: '=' },
        { label: '!=', value: '!=' },
        { label: '>', value: '>' },
        { label: '>=', value: '>=' },
        { label: '<', value: '<' },
        { label: '<=', value: '<=' },
        { label: 'like', value: 'like' },
        { label: 'left like', value: 'left like' },
        { label: 'right like', value: 'right like' }
      ],
      formTypeList: [
        { label: '单行文本', value: 'text' },
        { label: '多行文本', value: 'textarea' },
        { label: '富文本编辑器', value: 'editor' },
        { label: '下拉框', value: 'select' },
        { label: '单选按钮', value: 'radio' },
        { label: '复选框', value: 'checkbox' },
        { label: '日期', value: 'date' },
        { label: '日期时间', value: 'datetime' }
      ],
      tableId: '',
      tableData: []
    }
  },
  beforeDestroy () {
    if (this.sortable) {
      this.sortable.destroy()
    }
  },
  methods: {
    init (id) {
      this.visible = true
      this.$nextTick(() => {
        this.tableId = id
        this.getTable(id)
        this.getAttrTypeList()
        this.getDictList()
        this.rowDrop()
      })
    },
    rowDrop () {
      this.$nextTick(() => {
        const xTable = this.$refs.xTable
        this.sortable1 = Sortable.create(xTable.$el.querySelector('.body--wrapper>.vxe-table--body tbody'), {
          handle: '.drag-btn',
          onEnd: ({ newIndex, oldIndex }) => {
            const currRow = this.tableData.splice(oldIndex, 1)[0]
            this.tableData.splice(newIndex, 0, currRow)
          }
        })
      })
    },
    getTable (id) {
      this.$http.get('/devtools/table/' + id).then(({ data: res }) => {
        if (res.code !== 0) {
          return this.$message.error(res.msg)
        }
        this.tableData = res.data.fields
      }).catch(() => {})
    },
    getAttrTypeList () {
      this.$http.get('/devtools/fieldtype/list').then(({ data: res }) => {
        if (res.code !== 0) {
          return this.$message.error(res.msg)
        }
        this.typeList = []
        // 设置属性类型值
        res.data.forEach(item => this.typeList.push({ label: item, value: item }))
        // 增加Object类型
        this.typeList.push({ label: 'Object', value: 'Object' })
      }).catch(() => {})
    },
    getDictList () {
      this.$http.get('/devtools/dict').then(({ data: res }) => {
        if (res.code !== 0) {
          return this.$message.error(res.msg)
        }
        this.dictList = []
        this.dictList.push({ label: '', value: '' })
        res.data.forEach(item => this.dictList.push({ label: item.dictType, value: item.dictType }))
      }).catch(() => {})
    },
    // 修改
    updateHandle: debounce(function () {
      this.$http.put('/devtools/table/field/' + this.tableId, this.tableData).then(({ data: res }) => {
        if (res.code !== 0) {
          return this.$message.error(res.msg)
        }
        this.$message({
          message: this.$t('prompt.success'),
          type: 'success',
          duration: 500,
          onClose: () => {
            this.visible = false
            this.$emit('refreshDataList')
          }
        })
      }).catch(() => {})
    }, 1000, { leading: true, trailing: false })
  }
}
</script>

<style lang="scss">
.sortable-row-gen .drag-btn {
  cursor: move;
  font-size: 12px;
}
.sortable-row-gen .vxe-body--row.sortable-ghost,
.sortable-row-gen .vxe-body--row.sortable-chosen {
  background-color: #dfecfb;
}
</style>
