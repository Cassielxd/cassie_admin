<template>
  <el-card shadow="never" class="aui-card--fill">
    <div class="mod__process">
      <el-form :inline="true" :model="dataForm" @keyup.enter.native="getDataList()">
        <el-form-item>
          <el-input v-model="dataForm.processName" :placeholder="$t('process.name')" clearable></el-input>
        </el-form-item>
        <el-form-item>
          <el-input v-model="dataForm.key" :placeholder="$t('process.key')" clearable></el-input>
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
        <el-table-column prop="name" :label="$t('process.name')" header-align="center" align="center"></el-table-column>
        <el-table-column prop="key" :label="$t('process.key')" header-align="center" align="center"></el-table-column>
        <el-table-column prop="version" :label="$t('process.version')" header-align="center" align="center">
          <template slot-scope="scope">
            <el-tag size="small">v{{ scope.row.version }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="diagramResourceName" :label="$t('process.diagramResourceName')" header-align="center" align="center" :show-overflow-tooltip="true">
          <template slot-scope="scope">
            <a :href="getResourceURL(scope.row.deploymentId, scope.row.diagramResourceName)" target="_blank">{{ scope.row.diagramResourceName }}</a>
          </template>
        </el-table-column>
        <el-table-column prop="deploymentTime" :label="$t('process.deploymentTime')" header-align="center" align="center" width="180"></el-table-column>
        <el-table-column :label="$t('handle')" fixed="right" header-align="center" align="center" width="150">
          <template slot-scope="scope">
            <el-button type="text" size="small" @click="createProcessInstance(scope.row)">{{ $t('process.createInstance') }}</el-button>
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
      <add-or-update v-if="addOrUpdateVisible" ref="addOrUpdate" @refreshDataList="getDataList"></add-or-update>
    </div>
  </el-card>
</template>

<script>
import mixinViewModule from '@/mixins/view-module'
import Cookies from 'js-cookie'
import { addDynamicRoute } from '@/router'
import qs from 'qs'
export default {
  mixins: [mixinViewModule],
  data () {
    return {
      mixinViewModuleOptions: {
        getDataListURL: '/flow/common/start/page',
        getDataListIsPage: true
      },
      dataForm: {
        processName: '',
        key: ''
      }
    }
  },
  methods: {
    // 获取流程(image)url地址
    getResourceURL (id, name) {
      const params = qs.stringify({
        'access_token': Cookies.get('access_token'),
        'deploymentId': id,
        'resourceName': name
      })
      return `${window.SITE_CONFIG['apiURL']}/flow/process/resource?${params}`
    },
    // 发起流程
    createProcessInstance (row) {
      const routeParams = {
        routeName: `${this.$route.name}__instance_${row.deploymentId}`,
        menuId: `${this.$route.meta.menuId}`,
        title: `${row.name}`,
        path: `flow-form/${row.key}/form`,
        params: {
          processDefinitionId: row.id,
          processDefinitionKey: row.key
        }
      }
      addDynamicRoute(routeParams, this.$router, this.$route)
    }
  }
}
</script>
