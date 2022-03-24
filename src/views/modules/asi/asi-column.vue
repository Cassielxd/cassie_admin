<template>
  <el-card shadow="never" class="aui-card--fill">
  <el-container style="height: 100%; border: 1px solid #eee">
    <el-aside width="200px" style="background-color: rgb(238, 241, 246)">
      <el-menu>
        <sub-group v-for="group in groupList" @selectGroup="selectGroup" :key="group.group_code" :group="group"/>
      </el-menu>
    </el-aside>
    <el-container>

      <el-main>
        <el-form :inline="true" :model="dataForm" @keyup.enter.native="getDataList()">
          <el-form-item>
            <el-button @click="addOrUpdateHandle()">{{ $t('add') }}-{{ this.curarrGroup.name }}</el-button>
          </el-form-item>
        </el-form>
        <el-table :data="columsList" border style="width: 100%;">
          <el-table-column prop="column_name" label="列名" width="150">
          </el-table-column>
          <el-table-column prop="data_type" label="类型" width="150">
            <template slot-scope="scope">
              {{ $getDictLabel('asi_colums_type', scope.row.data_type) }}
            </template>
          </el-table-column>
          <el-table-column prop="column_code" label="列编码" width="150">
          </el-table-column>
          <el-table-column prop="is_required" label="是否必填" width="100"></el-table-column>
          <el-table-column prop="max_length" label="最大长度" width="100"></el-table-column>
          <el-table-column prop="group_code" label="业务分组code" width="150"></el-table-column>
          <el-table-column prop="agency_code" label="租户code" width="100">
          </el-table-column>
          <el-table-column :label="$t('handle')" fixed="right" header-align="center" align="center">
            <template slot-scope="scope">
              <el-button type="text" size="small" @click="addOrUpdateHandle(scope.row.id)">{{
                  $t('update')
                }}
              </el-button>
              <el-button type="text" @click="deleteHandle(scope.row.id)" size="small">{{ $t('delete') }}</el-button>
            </template>
          </el-table-column>
        </el-table>
      </el-main>
    </el-container>
    <add-or-update v-if="addOrUpdateVisible" ref="addOrUpdate" @refreshDataList="getColumsList"></add-or-update>
  </el-container>
  </el-card>
</template>

<script>
import SubGroup from './asi-sub-group'
import AddOrUpdate from './asi-column-add-or-update'

export default {
  data () {
    return {
      curarrGroup: {},
      addOrUpdateVisible: false,
      groupList: [],
      columsList: [],
      dataForm: {
        id: 0,
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
      // eslint-disable-next-line eqeqeq
      if (!this.curarrGroup.group_code || this.curarrGroup.group_code == '') {
        return this.$message.error('请选择一个业务分组')
      }
      this.$nextTick(() => {
        this.$refs.addOrUpdate.dataForm.id = id
        this.$refs.addOrUpdate.init(this.curarrGroup)
      })
    },
    deleteHandle (id) {
      this.$confirm(this.$t('prompt.info', { 'handle': this.$t('delete') }), this.$t('prompt.title'), {
        confirmButtonText: this.$t('confirm'),
        cancelButtonText: this.$t('cancel'),
        type: 'warning'
      }).then(() => {
        this.$http.delete('/asi/column/get_column_one/' + id
        ).then(({ data: res }) => {
          // eslint-disable-next-line eqeqeq
          if (res.code != 0) {
            return this.$message.error(res.msg)
          }
          this.$message({
            message: this.$t('prompt.success'),
            type: 'success',
            duration: 500,
            onClose: () => {
              this.getColumsList()
            }
          })
        }).catch(() => {
        })
      }).catch(() => {
      })
    },
    selectGroup (group) {
      // eslint-disable-next-line eqeqeq
      if (this.curarrGroup.group_code != group.group_code) {
        this.curarrGroup = group
        this.getColumsList()
      }

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
      return this.$http.get('/asi/column/list/' + this.curarrGroup.group_code).then(({ data: res }) => {
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
