<template>
  <el-card shadow="never" class="aui-card--fill">
    <div class="mod-pay__order">
      <el-form :inline="true" :model="dataForm" @keyup.enter.native="getDataList()">
        <el-form-item>
          <el-input v-model="dataForm.orderId" :placeholder="$t('order.orderId')" clearable></el-input>
        </el-form-item>
        <el-form-item>
          <el-input v-model="dataForm.userId" :placeholder="$t('order.userId')" clearable></el-input>
        </el-form-item>
        <el-form-item>
          <ren-select v-model="dataForm.status" dict-type="order_status" :placeholder="$t('order.status')"></ren-select>
        </el-form-item>
        <el-form-item>
          <el-button @click="getDataList()">{{ $t('query') }}</el-button>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="addOrUpdateHandle()">{{ $t('add') }}</el-button>
        </el-form-item>
      </el-form>
      <el-table v-loading="dataListLoading" :data="dataList" border @selection-change="dataListSelectionChangeHandle" style="width: 100%;">
        <el-table-column type="selection" header-align="center" align="center" width="50"></el-table-column>
        <el-table-column prop="orderId" :label="$t('order.orderId')" header-align="center" align="center"></el-table-column>
        <el-table-column prop="productName" :label="$t('order.productName')" header-align="center" align="center"></el-table-column>
        <el-table-column prop="payAmount" :label="$t('order.payAmount')" header-align="center" align="center"></el-table-column>
        <el-table-column prop="status" :label="$t('order.status')" header-align="center" align="center">
          <template slot-scope="scope">
            {{ $getDictLabel("order_status", scope.row.status) }}
          </template>
        </el-table-column>
        <el-table-column prop="payAt" :label="$t('order.payAt')" header-align="center" align="center"></el-table-column>
        <el-table-column prop="createDate" :label="$t('order.createDate')" header-align="center" align="center"></el-table-column>
        <el-table-column :label="$t('handle')" fixed="right" header-align="center" align="center" width="150">
          <template slot-scope="scope">
            <el-button v-if="scope.row.status === 0" type="text" size="small" @click="payHandle(scope.row.orderId)">{{ $t('order.pay') }}</el-button>
            <el-button v-if="scope.row.status === 0" type="text" size="small" @click="deleteHandle(scope.row.id)">{{ $t('delete') }}</el-button>
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
import AddOrUpdate from './order-add-or-update'
import {addDynamicRoute} from "@/router";
export default {
  mixins: [mixinViewModule],
  data () {
    return {
      mixinViewModuleOptions: {
        getDataListURL: '/sys/pay/order/page',
        getDataListIsPage: true,
        deleteURL: '/sys/pay/order',
        deleteIsBatch: true
      },
      dataForm: {
        orderId: '',
        status: '',
        userId: ''
      }
    }
  },
  components: {
    AddOrUpdate
  },
  methods: {
    payHandle (orderId) {
      window.open(`${window.SITE_CONFIG['apiURL']}/sys/pay/alipay/webPay?orderId=` + orderId);
    }
  }
}
</script>
