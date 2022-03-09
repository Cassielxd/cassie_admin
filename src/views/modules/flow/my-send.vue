<template>
  <el-card shadow="never" class="aui-card--fill">
    <div class="mod-__process">
      <el-form :inline="true" :model="dataForm" @keyup.enter.native="getDataList()">
        <el-form-item>
          <el-input v-model="dataForm.processDefinitionName" :placeholder="$t('process.name')" clearable></el-input>
        </el-form-item>
        <el-form-item>
          <el-button @click="getDataList()">{{ $t('query') }}</el-button>
        </el-form-item>
      </el-form>
      <el-table
        v-loading="dataListLoading"
        :data="dataList"
        border
        @selection-change="dataListSelectionChangeHandle"
        @sort-change="dataListSortChangeHandle"
        style="width: 100%;">
        <el-table-column type="selection" header-align="center" align="center" width="50"></el-table-column>
        <el-table-column prop="processDefinitionName" :label="$t('process.processDefinitionName')" header-align="center" align="center"></el-table-column>
        <el-table-column prop="taskName" label="当前节点" header-align="center" align="center"></el-table-column>
        <el-table-column prop="ended" :label="$t('process.ended')" header-align="center" align="center">
          <template slot-scope="scope">
            <el-tag v-if="scope.row.ended" size="small" type="success">{{ $t('process.ended0') }}</el-tag>
            <el-tag v-else size="small" type="danger">{{ $t('process.ended1') }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="startTime" :label="$t('process.startTime')" header-align="center" align="center">
          <template slot-scope="scope">
            <el-tag size="small">{{ scope.row.startTime }} ~ {{ scope.row.endTime }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="$t('handle')" fixed="right" header-align="center" align="center" width="150">
          <template slot-scope="scope">
            <el-button type="text" size="small" @click="showDetail(scope.row)">{{ $t('process.detail') }}</el-button>
          </template>
        </el-table-column>
      </el-table>
      <el-pagination
        :current-page="page"
        :page-sizes="[10, 20, 50, 100]"
        :page-size="limit"
        :total="total"
        layout="total, sizes, prev, pager, next, jumper"
        @size-change="pageSizeChangeHandle"
        @current-change="pageCurrentChangeHandle">
      </el-pagination>
    </div>
  </el-card>
</template>

<script>
import mixinViewModule from '@/mixins/view-module'
export default {
  mixins: [mixinViewModule],
  data () {
    return {
      mixinViewModuleOptions: {
        getDataListURL: '/flow/common/my/page',
        getDataListIsPage: true
      },
      dataForm: {
        processDefinitionName: ''
      }
    }
  },
  methods: {
    showDetail (row) {
      if (!row.businessKey) {
        return this.$message.error(this.$t('task.detailError'))
      }
      this.flowDetailRoute(row)
    }
  }
}
</script>
