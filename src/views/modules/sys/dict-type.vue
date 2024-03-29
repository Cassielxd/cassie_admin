<template>
  <el-card shadow="never" class="aui-card--fill">
    <div class="mod-sys__dict">
      <el-form :inline="true" :model="dataForm" @keyup.enter.native="getDataList()">
        <el-form-item>
          <el-input v-model="dataForm.dict_name" :placeholder="$t('dict.dictName')" clearable></el-input>
        </el-form-item>
        <el-form-item>
          <el-input v-model="dataForm.dict_type" :placeholder="$t('dict.dictType')" clearable></el-input>
        </el-form-item>
        <el-form-item>
          <el-button @click="getDataList()">{{ $t('query') }}</el-button>
        </el-form-item>
        <el-form-item>
          <el-button @click="addOrUpdateHandle()">{{ $t('add') }}</el-button>
        </el-form-item>
        <el-form-item>
          <el-button  type="danger" @click="deleteHandle()">{{ $t('deleteBatch') }}</el-button>
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
        <el-table-column prop="dict_name" :label="$t('dict.dictName')" header-align="center" align="center"></el-table-column>
        <el-table-column prop="dict_type" :label="$t('dict.dictType')" header-align="center" align="center">
          <template slot-scope="scope">
            <el-button type="text" @click="childHandle(scope.row)">{{ scope.row.dict_type }}</el-button>
          </template>
        </el-table-column>
        <el-table-column prop="sort" :label="$t('dict.sort')" sortable="custom" header-align="center" align="center"></el-table-column>
        <el-table-column prop="remark" :label="$t('dict.remark')" header-align="center" align="center"></el-table-column>
        <el-table-column prop="create_date" :label="$t('dict.createDate')" sortable="custom" header-align="center" align="center" width="180"></el-table-column>
        <el-table-column  :label="$t('handle')" fixed="right" header-align="center" align="center" width="150">
          <template slot-scope="scope">
            <el-button  type="text" size="small" @click="addOrUpdateHandle(scope.row.id)">{{ $t('update') }}</el-button>
            <el-button  type="text" size="small" @click="deleteHandle(scope.row.id)">{{ $t('delete') }}</el-button>
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
      <!-- 弹窗, 新增 / 修改 -->
      <add-or-update v-if="addOrUpdateVisible" ref="addOrUpdate" @refreshDataList="getDataList"></add-or-update>
    </div>
  </el-card>
</template>

<script>
import mixinViewModule from '@/mixins/view-module'
import AddOrUpdate from './dict-type-add-or-update'
import { addDynamicRoute } from '@/router'
export default {
  mixins: [mixinViewModule],
  data () {
    return {
      mixinViewModuleOptions: {
        getDataListURL: '/dict/type',
        getDataListIsPage: true,
        deleteURL: '/dict/type',
        deleteIsBatch: true
      },
      dataForm: {
        id: '0',
        dict_name: '',
        dict_type: ''
      }
    }
  },
  components: {
    AddOrUpdate
  },
  methods: {
    // 子级
    childHandle (row) {
      // 路由参数
      const routeParams = {
        routeName: `${this.$route.name}__${row.id}`,
        title: `${this.$route.meta.title} - ${row.dict_name}`,
        path: 'sys/dict-data',
        params: {
          dictTypeId: row.id
        }
      }
      // 动态路由
      addDynamicRoute(routeParams, this.$router)
    }
  }
}
</script>
