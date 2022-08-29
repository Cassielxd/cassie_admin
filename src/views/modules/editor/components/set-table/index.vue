<template>
    <vxe-table
        border
        row-key
        ref="xTable"
        class="sortable-row-gen"
        size="mini"
        :data="value"
        :checkbox-config="{checkStrictly: true}"
        :edit-config="{trigger: 'click', mode: 'cell', icon: 'fa fa-pencil'}">
      <vxe-table-column field="columnCode" width="75" title="列编码">
        <template v-slot="{ row }">
          <vxe-input v-model="row.columnCode" type="text"></vxe-input>
        </template>
      </vxe-table-column>
      <vxe-table-column field="columnName" width="75" title="列名称">
        <template v-slot="{ row }">
          <vxe-input v-model="row.columnName" type="text"></vxe-input>
        </template>
      </vxe-table-column>
      <vxe-table-column field="width" width="75" title="列宽">
        <template v-slot="{ row }">
          <vxe-input v-model="row.width" type="text"></vxe-input>
        </template>
      </vxe-table-column>

    </vxe-table>
</template>

<script>

export default {

  props: {
    value: Array,
  },
  data(){
    return {
      visible: false,
    };
  },
  methods: {
    updateHandle(){
        this.visible=true;
    }
  }
}
</script>

<style >

</style>
