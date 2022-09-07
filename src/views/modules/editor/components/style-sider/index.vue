<template>
  <el-tabs type="card" v-model="edit">
    <el-tab-pane label="属性" name="style">
      <el-form
          v-if="current"
          :model="styles"
          :inline="false"
      >
        <el-form-item
            v-for="formItem in form.styleForm"
            :key="formItem.key"
            :label="formItem.label"
        >
          <component
              v-model="styles[formItem.key]"
              :is="formItem.component"
          />
        </el-form-item>
<!--        <el-form-item>
          <el-button @click="$emit('change', current.id, styles)">保存</el-button>
        </el-form-item>-->
      </el-form>

    </el-tab-pane>
    <el-tab-pane label="列信息" name="column" v-if="form.type =='table'">
      <vxe-button icon="vxe-icon-square-plus" @click="insertEvent()">新增</vxe-button>
      <vxe-table
          border
          row-key
          ref="xTable"
          class="sortable-row-gen"
          size="mini"
          :data="styles.tableColumns"
          :checkbox-config="{checkStrictly: true}"
          :edit-config="{trigger: 'dblclick', mode: 'row'}">
        <vxe-table-column field="columnCode" width="75" title="列编码" :edit-render="{autofocus: '.vxe-input--inner'}">
          <template #edit="{ row }">
            <vxe-input v-model="row.columnCode" type="text"></vxe-input>
          </template>
        </vxe-table-column>
        <vxe-table-column field="columnName" width="75" title="列名称" :edit-render="{autofocus: '.vxe-input--inner'}">
          <template #edit="{ row }">
            <vxe-input v-model="row.columnName" type="text"></vxe-input>
          </template>
        </vxe-table-column>
        <vxe-table-column field="width" width="75" title="列宽" :edit-render="{autofocus: '.vxe-input--inner'}">
          <template #edit="{ row }">
            <vxe-input v-model="row.width" type="text"></vxe-input>
          </template>
        </vxe-table-column>
        <vxe-table-column title="操作" width="75">
          <template #default="{ row }">
            <vxe-button status="danger" content="直接删除" @click="deleteSelectEvent(row)"></vxe-button>
          </template>
        </vxe-table-column>
      </vxe-table>
    </el-tab-pane>
  </el-tabs>


</template>

<script>
import SetTable from '../set-table'

export default {
  components: {
    SetTable
  },
  props: {
    current: {
      type: Object,
      default () {
        return null
      },
    },
    form: {
      type: Object,
      default () {
        return {}
      },
    },
  },
  watch: {
    current (newVal) {
      if (newVal) {
        this.styles = newVal.styles
      }
    },
  },
  data () {
    return {
      edit: 'style',
      styles: {},
      tableColumns: []
    }
  },
  methods:{
    async insertEvent (row) {
      const $table = this.$refs.xTable
      const record = {
        columnCode: 'new',
        columnName: '',
        width:'180'
      }
      this.styles.tableColumns.push(record);
      const { row: newRow } = await $table.insertAt(record, row)
      await $table.setActiveCell(newRow, 'name')
    },
    async deleteSelectEvent (row) {
      const $table = this.$refs.xTable
      this.styles.tableColumns=this.styles.tableColumns.filter(item=>item.columnCode!=row.columnCode)

      await $table.remove(row)
    },
  }
}
</script>

<style>
.el-form-item {
  margin-bottom: 1px;
}
</style>
