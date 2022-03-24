<template>
  <el-card shadow="never" class="aui-card--fill">
  <el-container style="height: 500px; border: 1px solid #eee">

    <el-aside width="200px" style="background-color: rgb(238, 241, 246)">
      <el-menu>
        <sub-group v-for="group in groupList" @selectGroup="selectGroup" :key="group.group_code" :group="group"/>
      </el-menu>
    </el-aside>
    <el-container>
      <el-main>
        <el-table :data="dataList" border style="width: 100%;">
          <el-table-column v-for="column in columsList" :prop="column.column_code" :label="column.column_name"
                           :key="column.column_code"
          >
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
      </el-main>
    </el-container>

  </el-container>
  </el-card>
</template>

<script>
import SubGroup from './asi-sub-group'
import AddOrUpdate from './asi-column-add-or-update'
import mixinViewModule from '@/mixins/view-module'

export default {
  mixins: [mixinViewModule],
  data () {
    return {
      mixinViewModuleOptions: {
        getDataListURL: '/asi/group',
        getDataListIsPage: true,
        deleteURL: '/asi/group',
        deleteIsBatch: false,
        createdIsNeed: false,
        activatedIsNeed: false
      },
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
    selectGroup (group) {
      // eslint-disable-next-line eqeqeq
      if (this.curarrGroup.group_code != group.group_code) {
        this.curarrGroup = group
        this.dataForm.group_code = group.group_code
        this.getColumsList()
        this.query()
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
    },
    getColumsValueList () {

    }
  }
}
</script>
