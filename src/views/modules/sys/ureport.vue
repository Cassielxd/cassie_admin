<template>
  <el-card shadow="never" class="aui-card--fill">
    <div class="mod-ureport__model">
      <el-form :inline="true" :model="dataForm" @keyup.enter.native="getDataList()">
        <el-form-item>
          <el-input v-model="dataForm.fileName" :placeholder="$t('fileName')" clearable></el-input>
        </el-form-item>
        <el-form-item>
          <el-button @click="getDataList()">{{ $t('query') }}</el-button>
        </el-form-item>
        <el-form-item>
          <el-button type="danger" @click="deleteHandle()">{{ $t('deleteBatch') }}</el-button>
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
        <el-table-column prop="fileName" :label="$t('fileName')" header-align="center" align="center"></el-table-column>
        <el-table-column prop="createDate" :label="$t('createDate')" header-align="center" align="center"></el-table-column>
        <el-table-column prop="updateDate" :label="$t('updateDate')" header-align="center" align="center"></el-table-column>
        <el-table-column :label="$t('handle')" fixed="right" header-align="center" align="center">
          <template slot-scope="scope">
            <a href="javascript:;" @click="$router.push({ name: 'iframe', query: { key: 'designer-'+scope.row.fileName, menuId: $route.meta.menuId, url: getDesignerURL(scope.row.fileName) } })" class="el-button el-button--text el-button--small">{{ $t('design') }}</a>
            <a href="javascript:;" @click="$router.push({ name: 'iframe', query: { key: 'preview-'+scope.row.fileName, menuId: $route.meta.menuId, url: getPreviewURL(scope.row.fileName) } })" class="el-button el-button--text el-button--small">{{ $t('preview') }}</a>
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
      <!-- 弹窗, 新增 / 修改 -->
      <add-or-update v-if="addOrUpdateVisible" ref="addOrUpdate" @refreshDataList="getDataList"></add-or-update>
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
        getDataListURL: '/sys/ureport2/page',
        getDataListIsPage: true,
        deleteURL: '/sys/ureport2',
        deleteIsBatch: true
      },
      dataForm: {
        fileName: ''
      }
    }
  },
  methods: {
    // 设计器url地址
    getDesignerURL (fileName) {
      return `${window.SITE_CONFIG['apiURL']}/sys/ureport/designer?_u=renren-${fileName}`
    },
    // 预览url地址
    getPreviewURL (fileName) {
      return `${window.SITE_CONFIG['apiURL']}/sys/ureport/preview?_u=renren-${fileName}`
    }
  }
}
</script>
