<template>
  <el-card shadow="never" class="aui-card--fill">
      <div style="text-align: center;font-size: 28px">{{dataForm.title}}</div>
      <el-divider></el-divider>
      <div v-html="dataForm.content"></div>
      <div>
          <hr size=1 color="#ddd" style="margin:30px 0 10px 0;">
          <span><i class="el-icon-user-solid" style="color: #e6444a"></i> {{ $t('notice.senderName') }}：{{dataForm.senderName}}</span>
          <el-divider direction="vertical"></el-divider>
          <span><i class="el-icon-time" style="color: #e6444a"></i> {{ $t('notice.senderDate') }}：{{dataForm.senderDate}}</span>
          <el-divider direction="vertical" style="margin: 0px;padding:0px;"></el-divider>
          <span><i class="el-icon-s-order" style="color: #E6A23C"></i> {{ $t('notice.type') }}：
              <template>
              {{ $getDictLabel("notice_type", dataForm.type) }}
              </template>
          </span>
          <hr  size=1 color="#ddd" style="margin:10px 0 30px 0;">
      </div>
      <el-table v-loading="dataListLoading" :data="dataList" border @selection-change="dataListSelectionChangeHandle" style="width: 100%;">
          <el-table-column prop="receiverName" :label="$t('notice.receiverName')" header-align="center" align="center"></el-table-column>
          <el-table-column prop="readStatus" :label="$t('notice.readStatus')" header-align="center" align="center">
              <template slot-scope="scope">
                  <el-tag v-if="scope.row.readStatus === 0" size="small" type="danger">{{ $t('notice.readStatus0') }}</el-tag>
                  <el-tag v-else size="small" type="success">{{ $t('notice.readStatus1') }}</el-tag>
              </template>
          </el-table-column>
          <el-table-column prop="readDate" :label="$t('notice.readDate')" header-align="center" align="center"></el-table-column>
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
      <el-divider></el-divider>
      <div style="text-align: center;"><el-button type="danger" @click="closeCurrentTab()">{{ $t('notice.close') }}</el-button></div>
  </el-card>
</template>

<script>
import mixinViewModule from '@/mixins/view-module'
export default {
  mixins: [mixinViewModule],
  data () {
    return {
      mixinViewModuleOptions: {
        getDataListURL: '/sys/notice/user/page',
        createdIsNeed: false,
        activatedIsNeed: true,
        getDataListIsPage: true
      },
      dataForm: {
        id: ''
      }
    }
  },
  created () {
    this.dataForm.id = this.$route.params.id || 0
    this.getInfo()
  },
  methods: {
    // 获取信息
    getInfo () {
      this.$http.get(`/sys/notice/${this.dataForm.id}`).then(({ data: res }) => {
        if (res.code !== 0) {
          return this.$message.error(res.msg)
        }
        this.dataForm = {
          ...this.dataForm,
          ...res.data
        }
      }).catch(() => {})
    }
  }
}
</script>
