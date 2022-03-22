<template>
  <el-container style="height: 500px; border: 1px solid #eee">
    <el-aside width="200px" style="background-color: rgb(238, 241, 246)">
      <el-menu>
        <sub-group v-for="group in groupList" @selectGroup="selectGroup" :key="group.group_code" :group="group"/>
      </el-menu>
    </el-aside>
    <el-container>

      <el-main>
        <el-form :inline="true" :model="dataForm" @keyup.enter.native="getDataList()">
          <el-form-item>
            <el-button @click="addOrUpdateHandle()">{{ $t('add') }}</el-button>
          </el-form-item>
        </el-form>
        <el-table :data="columsList">
          <el-table-column prop="column_name" label="列名" width="200">
          </el-table-column>
          <el-table-column prop="data_type" label="类型" width="200">
          </el-table-column>
          <el-table-column prop="column_code" label="列编码" width="200">
          </el-table-column>
          <el-table-column prop="is_required" label="是否必填" width="200"></el-table-column>
          <el-table-column prop="group_code" label="业务分组code" width="200"></el-table-column>
          <el-table-column prop="agency_code" label="租户code" width="200">
          </el-table-column>
          <el-table-column :label="$t('handle')" fixed="right" header-align="center" align="center">
            <template slot-scope="scope">
              <el-button type="text" size="small">{{ $t('delete') }}</el-button>
            </template>
          </el-table-column>
        </el-table>
      </el-main>
    </el-container>
    <add-or-update v-if="addOrUpdateVisible" ref="addOrUpdate" @refreshDataList="getColumsList"></add-or-update>
  </el-container>
</template>

<script>
import SubGroup from './asi-sub-group'
import AddOrUpdate from './asi-column-add-or-update'
export default {
  data () {
    return {
      addOrUpdateVisible: false,
      groupList: [],
      columsList: [],
      dataForm: {
        id: 0,
        group_name: '',
        group_code: ''
      }
    }
  },
  components: {
    AddOrUpdate,
    SubGroup
  },
  created () {
    this.getGroupList()
  },
  methods: {
    addOrUpdateHandle (id) {
      this.addOrUpdateVisible = true
      this.$nextTick(() => {
        this.$refs.addOrUpdate.dataForm.id = id
        this.$refs.addOrUpdate.init()
      })
    },
    selectGroup (groupCode) {
      // eslint-disable-next-line eqeqeq
      if (this.dataForm.group_code != groupCode) {
        this.dataForm.group_code = groupCode
        this.getColumsList()
      }

      console.log(groupCode)
    },
    getGroupList () {
      return this.$http.get('/asi/group/list').then(({ data: res }) => {
        // eslint-disable-next-line eqeqeq
        if (res.code != 0) {
          return this.$message.error(res.msg)
        }
        this.groupList = res.data
      }).catch(() => {
      })
    },
    getColumsList () {
      return this.$http.get('/asi/column/list/' + this.dataForm.group_code).then(({ data: res }) => {
        // eslint-disable-next-line eqeqeq
        if (res.code != 0) {
          return this.$message.error(res.msg)
        }
        this.columsList = res.data
      }).catch(() => {
      })
    }
  }
}
</script>
