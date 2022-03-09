<template>
  <el-card shadow="never" class="aui-card--fill">
    <div class="mod-generator">
      <el-form :inline="true" :model="dataForm" @keyup.enter.native="getDataList()">
        <el-form-item>
          <el-input v-model="dataForm.tableName" placeholder="表名"></el-input>
        </el-form-item>
        <el-form-item>
          <el-button @click="getDataList()">查询</el-button>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="importHandle()">导入数据库表</el-button>
        </el-form-item>
        <el-form-item>
          <el-button type="danger" @click="deleteHandle()">删除</el-button>
        </el-form-item>
      </el-form>
      <el-table v-loading="dataListLoading" :data="dataList" border @selection-change="dataListSelectionChangeHandle" style="width: 100%;">
        <el-table-column type="selection" header-align="center" align="center" width="50"></el-table-column>
        <el-table-column prop="tableName" label="表名" header-align="center" align="center"></el-table-column>
        <el-table-column prop="tableComment" label="表说明" header-align="center" align="center"></el-table-column>
        <el-table-column prop="className" label="类名" header-align="center" align="center"></el-table-column>
        <el-table-column prop="createDate" label="创建时间" header-align="center" align="center"></el-table-column>
        <el-table-column label="操作" fixed="right" header-align="center" align="center" width="250">
          <template slot-scope="scope">
            <el-button type="text" size="small" @click="editTableHandle(scope.row.id)">编辑</el-button>
            <el-button type="text" size="small" @click="generatorCodeHandle(scope.row.id)">生成代码</el-button>
            <el-button type="text" size="small" @click="generatorMenuHandle(scope.row)">创建菜单</el-button>
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
      <import v-if="importVisible" ref="import" @refreshDataList="getDataList"></import>
      <edit-table v-if="editTableVisible" ref="editTable" @refreshDataList="getDataList"></edit-table>
      <generator-code v-if="generatorCodeVisible" ref="generatorCode" @refreshDataList="getDataList"></generator-code>
      <generator-menu v-if="generatorMenuVisible" ref="generatorMenu" @refreshDataList="getDataList"></generator-menu>
    </div>
  </el-card>
</template>

<script>
import mixinViewModule from '@/mixins/view-module'
import Import from './generator-import'
import EditTable from './generator-edittable'
import GeneratorCode from './generator-code'
import GeneratorMenu from './generator-menu'
export default {
  mixins: [mixinViewModule],
  data () {
    return {
      mixinViewModuleOptions: {
        getDataListURL: '/devtools/table/page',
        getDataListIsPage: true,
        deleteURL: '/devtools/table',
        deleteIsBatch: true
      },
      importVisible: false,
      editTableVisible: false,
      generatorCodeVisible: false,
      generatorMenuVisible: false,
      dataForm: {
        tableName: ''
      }
    }
  },
  components: {
    Import,
    EditTable,
    GeneratorCode,
    GeneratorMenu
  },
  methods: {
    importHandle: function () {
      this.importVisible = true
      this.$nextTick(() => {
        this.$refs.import.init()
      })
    },
    editTableHandle: function (id) {
      this.editTableVisible = true
      this.$nextTick(() => {
        this.$refs.editTable.init(id)
      })
    },
    generatorCodeHandle: function (id) {
      this.generatorCodeVisible = true
      this.$nextTick(() => {
        this.$refs.generatorCode.init(id)
      })
    },
    generatorMenuHandle: function (row) {
      this.generatorMenuVisible = true
      this.$nextTick(() => {
        this.$refs.generatorMenu.dataForm.backendUrl = row.backendUrl
        this.$refs.generatorMenu.dataForm.className = row.className
        this.$refs.generatorMenu.init()
      })
    }
  }
}
</script>
