<template>
  <el-card shadow="never" class="aui-card--fill">
    <div class="mod-pay__alipaynotifylog">
      <el-form :inline="true" :model="dataForm" @keyup.enter.native="getDataList()">
        <el-form-item>
          <el-input v-model="dataForm.outTradeNo" :placeholder="$t('order.outTradeNo')" clearable></el-input>
        </el-form-item>
        <el-form-item>
          <el-input v-model="dataForm.notifyId" :placeholder="$t('order.notifyId')" clearable></el-input>
        </el-form-item>
        <el-form-item>
          <el-input v-model="dataForm.tradeStatus" :placeholder="$t('order.tradeStatus')" clearable></el-input>
        </el-form-item>
        <el-form-item>
          <el-button @click="getDataList()">{{ $t('query') }}</el-button>
        </el-form-item>
      </el-form>
      <el-table v-loading="dataListLoading" :data="dataList" border @selection-change="dataListSelectionChangeHandle" style="width: 100%;">
        <el-table-column type="selection" header-align="center" align="center" width="50"></el-table-column>
        <el-table-column prop="outTradeNo" :label="$t('order.orderId')" header-align="center" align="center"></el-table-column>
        <el-table-column prop="totalAmount" :label="$t('order.totalAmount')" header-align="center" align="center"></el-table-column>
        <el-table-column prop="buyerPayAmount" :label="$t('order.buyerPayAmount')" header-align="center" align="center"></el-table-column>
        <el-table-column prop="notifyId" :label="$t('order.notifyId')" header-align="center" align="center"></el-table-column>
        <el-table-column prop="tradeNo" :label="$t('order.tradeNo')" header-align="center" align="center"></el-table-column>
        <el-table-column prop="tradeStatus" :label="$t('order.tradeStatus')" header-align="center" align="center"></el-table-column>
        <el-table-column prop="createDate" :label="$t('createDate')" header-align="center" align="center"></el-table-column>
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
export default {
  mixins: [mixinViewModule],
  data () {
    return {
      mixinViewModuleOptions: {
        getDataListURL: '/sys/pay/alipayNotifyLog/page',
        getDataListIsPage: true,
        deleteIsBatch: true
      },
      dataForm: {
        outTradeNo: '',
        notifyId: '',
        tradeStatus: ''
      }
    }
  }
}
</script>
