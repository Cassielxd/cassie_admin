<template>
  <el-card shadow="never" class="aui-card--fill">
    <div class="mod-demo__excel">
      <el-form :inline="true" :model="dataForm" @keyup.enter.native="getDataList()">
        <el-form-item>
          <el-input v-model="dataForm.realName" :placeholder="$t('excel.realName')" clearable></el-input>
        </el-form-item>
        <el-form-item>
          <el-input v-model="dataForm.identity" :placeholder="$t('excel.identity')" clearable></el-input>
        </el-form-item>
        <el-form-item>
          <el-button @click="getDataList()">{{ $t('query') }}</el-button>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="addOrUpdateHandle()">{{ $t('add') }}</el-button>
        </el-form-item>
        <el-form-item>
          <el-button type="danger" @click="deleteHandle()">{{ $t('deleteBatch') }}</el-button>
        </el-form-item>
        <el-form-item>
          <el-button type="warning" @click="importHandle()">{{ $t('excel.import') }}</el-button>
        </el-form-item>
        <el-form-item>
          <el-button type="info" @click="exportHandle()">{{ $t('export') }}</el-button>
        </el-form-item>
      </el-form>
      <el-table v-loading="dataListLoading" :data="dataList" border @selection-change="dataListSelectionChangeHandle" style="width: 100%;">
        <el-table-column type="selection" header-align="center" align="center" width="50"></el-table-column>
        <el-table-column prop="realName" :label="$t('excel.realName')" header-align="center" align="center"></el-table-column>
        <el-table-column prop="identity" :label="$t('excel.identity')" header-align="center" align="center"></el-table-column>
        <el-table-column prop="address" :label="$t('excel.address')" header-align="center" align="center"></el-table-column>
        <el-table-column prop="joinDate" :label="$t('excel.joinDate')" header-align="center" align="center"></el-table-column>
        <el-table-column prop="className" :label="$t('excel.className')" header-align="center" align="center"></el-table-column>
        <el-table-column :label="$t('handle')" fixed="right" header-align="center" align="center" width="150">
          <template slot-scope="scope">
            <el-button type="text" size="small" @click="addOrUpdateHandle(scope.row.id)">{{ $t('update') }}</el-button>
            <el-button type="text" size="small" @click="deleteHandle(scope.row.id)">{{ $t('delete') }}</el-button>
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
      <import v-if="importVisible" ref="import" @refreshDataList="getDataList"></import>
      <!-- 弹窗, 新增 / 修改 -->
      <add-or-update v-if="addOrUpdateVisible" ref="addOrUpdate" @refreshDataList="getDataList"></add-or-update>
    </div>
  </el-card>
</template>

<script>
import mixinViewModule from '@/mixins/view-module'
import Import from './excel-import'
import AddOrUpdate from './excel-add-or-update'
export default {
  mixins: [mixinViewModule],
  data () {
    return {
      mixinViewModuleOptions: {
        getDataListURL: '/sys/excel/page',
        getDataListIsPage: true,
        exportURL: '/sys/excel/export',
        deleteURL: '/sys/excel',
        deleteIsBatch: true
      },
      importVisible: false,
      dataForm: {
        realName: '',
        identity: '',
      }
    }
  },
  components: {
    Import,
    AddOrUpdate
  },
  methods: {
    importHandle: function () {
      this.importVisible = true
      this.$nextTick(() => {
        this.$refs.import.init()
      })
    }
  }
}
</script>
