<template>
  <el-card shadow="never" class="aui-card--fill">
    <div class="mod-demo__sysnoticeuser">
      <el-form :inline="true" :model="dataForm" @keyup.enter.native="getDataList()">
        <el-form-item>
          <ren-select v-model="dataForm.type" dict-type="notice_type" :placeholder="$t('notice.type')"></ren-select>
        </el-form-item>
        <el-form-item>
          <el-button @click="getDataList()">{{ $t('query') }}</el-button>
        </el-form-item>
      </el-form>
      <el-table v-loading="dataListLoading" :data="dataList" border @selection-change="dataListSelectionChangeHandle" style="width: 100%;">
        <el-table-column type="selection" header-align="center" align="center" width="50"></el-table-column>
        <el-table-column prop="title" :label="$t('notice.title')" header-align="center" align="center"></el-table-column>
        <el-table-column prop="type" :label="$t('notice.type')" header-align="center" align="center" width="150">
            <template slot-scope="scope">
                {{ $getDictLabel("notice_type", scope.row.type) }}
            </template>
        </el-table-column>
        <el-table-column prop="senderName" :label="$t('notice.senderName')" header-align="center" align="center" width="150"></el-table-column>
        <el-table-column prop="senderDate" :label="$t('notice.senderDate')" header-align="center" align="center" width="170"></el-table-column>
        <el-table-column prop="readStatus" :label="$t('notice.readStatus')" header-align="center" align="center" width="130">
          <template slot-scope="scope">
              <el-tag v-if="scope.row.readStatus === 0" size="small" type="danger">{{ $t('notice.readStatus0') }}</el-tag>
              <el-tag v-else size="small" type="success">{{ $t('notice.readStatus1') }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="$t('handle')" fixed="right" header-align="center" align="center" width="150">
          <template slot-scope="scope">
            <el-button type="text" size="small" @click="viewHandle(scope.row)">{{ $t('notice.view') }}</el-button>
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
import { addDynamicRoute } from '@/router'
export default {
  mixins: [mixinViewModule],
  data () {
    return {
      mixinViewModuleOptions: {
        getDataListURL: '/sys/notice/mynotice/page',
        getDataListIsPage: true
      },
      dataForm: {
        type: ''
      }
    }
  },
  methods: {
    viewHandle (row) {
      // 路由参数
      const routeParams = {
        routeName: `${this.$route.name}__${row.id}`,
        title: this.$t('notice.view2'),
        path: 'sys/notice-user-view',
        params: {
          id: row.id
        }
      }

      // 如果未读，则标记为已读
      if (row.readStatus === 0) {
        this.updateReadStatus(row.id)
      }

      // 动态路由
      addDynamicRoute(routeParams, this.$router)
    },
    updateReadStatus (noticeId) {
      this.$http['put']('/sys/notice/mynotice/read/' + noticeId).then(({ data: res }) => {
        if (res.code !== 0) {
          return this.$message.error(res.msg)
        }
      }).catch(() => {})
    }
  }
}
</script>
