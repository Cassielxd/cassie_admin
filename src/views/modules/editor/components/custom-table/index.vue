<template>
  <el-table
      :data="value.tableData"
      :stripe="styles.stripe"
      style="width: 100%">
    <el-table-column v-for="item in styles.tableColumns"
        :prop="item.columnCode"
        :label="item.columnName"
        :width="item.width">
    </el-table-column>
  </el-table>
</template>

<script>
export default {
  props: {
    value: Object,
    styles: Object,
  },
  data(){
    return {};
  },
  methods: {
  }
}
</script>

<style lang="less" scoped>

</style>
