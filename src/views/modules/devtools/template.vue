<template>
  <el-card shadow="never" class="aui-card--fill">
    <div class="mod-template">
      <el-form :inline="true" :model="dataForm" @keyup.enter.native="getDataList()">
        <el-form-item>
          <el-input v-model="dataForm.name" placeholder="模板名" clearable></el-input>
        </el-form-item>
        <el-form-item>
          <el-button @click="getDataList()">查询</el-button>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="addOrUpdateHandle()">新增</el-button>
        </el-form-item>
        <el-form-item>
          <el-button type="success" @click="enabledHandle()">启用</el-button>
        </el-form-item>
        <el-form-item>
          <el-button type="warning" @click="disabledHandle()">禁用</el-button>
        </el-form-item>
        <el-form-item>
          <el-button type="danger" @click="deleteHandle()">删除</el-button>
        </el-form-item>
      </el-form>
      <el-table v-loading="dataListLoading" :data="dataList" border @selection-change="dataListSelectionChangeHandle" style="width: 100%;">
        <el-table-column type="selection" header-align="center" align="center" width="50"></el-table-column>
        <el-table-column prop="name" label="模板名" header-align="center" align="center" width="150"></el-table-column>
        <el-table-column prop="fileName" label="文件名" header-align="center" align="center" width="200"></el-table-column>
        <el-table-column prop="path" label="生成路径" header-align="center"></el-table-column>
        <el-table-column prop="status" label="状态" sortable="custom" header-align="center" align="center" width="120">
          <template slot-scope="scope">
            <el-tag v-if="scope.row.status === 0" size="small" type="success">启用</el-tag>
            <el-tag v-else size="small" type="danger">禁用</el-tag>
          </template>
        </el-table-column>
        <el-table-column label="操作" fixed="right" header-align="center" align="center" width="120">
          <template slot-scope="scope">
            <el-button type="text" size="small" @click="addOrUpdateHandle(scope.row.id)">编辑</el-button>
            <el-button type="text" size="small" @click="deleteHandle(scope.row.id)">删除</el-button>
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
import AddOrUpdate from './template-add-or-update'
export default {
  mixins: [mixinViewModule],
  data () {
    return {
      mixinViewModuleOptions: {
        getDataListURL: '/devtools/template/page',
        getDataListIsPage: true,
        deleteURL: '/devtools/template',
        deleteIsBatch: true
      },
      dataForm: {
        name: ''
      }
    }
  },
  methods: {
    enabledHandle (id) {
      if (!id && this.dataListSelections.length <= 0) {
        return this.$message({
          message: '请选择记录',
          type: 'warning',
          duration: 500
        })
      }
      this.$http.put('/devtools/template/enabled', id ? [id] : this.dataListSelections.map(item => item.id)).then(({data: res}) => {
        if (res.code !== 0) {
          return this.$message.error(res.msg)
        }
        this.$message({
          message: this.$t('prompt.success'),
          type: 'success',
          duration: 500,
          onClose: () => {
            this.getDataList()
          }
        })
      }).catch(() => {
      })
    },
    disabledHandle (id) {
      if (!id && this.dataListSelections.length <= 0) {
        return this.$message({
          message: '请选择记录',
          type: 'warning',
          duration: 500
        })
      }
      this.$http.put('/devtools/template/disabled', id ? [id] : this.dataListSelections.map(item => item.id)).then(({data: res}) => {
        if (res.code !== 0) {
          return this.$message.error(res.msg)
        }
        this.$message({
          message: this.$t('prompt.success'),
          type: 'success',
          duration: 500,
          onClose: () => {
            this.getDataList()
          }
        })
      }).catch(() => {
      })
    }
  },
  components: {
    AddOrUpdate
  }
}
</script>
