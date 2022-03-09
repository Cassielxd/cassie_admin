<template>
  <el-dialog :visible.sync="visible" title="导入数据库表" :close-on-click-modal="false" :close-on-press-escape="false">
    <el-form :model="dataForm" :rules="dataRule" ref="dataForm" label-width="120px">
      <el-form-item label="数据源" prop="datasourceId">
        <el-select v-model="dataForm.datasourceId" @change="getTableInfoList" style="width:100%" placeholder="请选择数据源">
          <el-option label="默认数据源" value="0"></el-option>
          <el-option
              v-for="ds in dataForm.datasourceList"
              :key="ds.id"
              :label="ds.connName"
              :value="ds.id">
          </el-option>
        </el-select>
      </el-form-item>
      <el-form-item label="选择表" prop="tableInfo">
        <el-select v-model="dataForm.tableInfo" value-key="tableName" :disabled="!dataForm.showTableSelect" style="width:100%" placeholder="请选择表名">
          <el-option
              v-for="tableInfo in dataForm.tableInfoList"
              :key="tableInfo.tableName"
              :label="tableInfo.tableName"
              :value="tableInfo">
          </el-option>
        </el-select>
      </el-form-item>
    </el-form>
    <template slot="footer">
      <el-button @click="visible = false">取消</el-button>
      <el-button type="primary" @click="dataFormSubmitHandle">确定</el-button>
    </template>
  </el-dialog>
</template>
<script>
import debounce from 'lodash/debounce'
export default {
  data () {
    return {
      visible: false,
      dataForm: {
        id: '',
        datasourceId: '',
        datasourceList: [],
        tableInfoList: [],
        tableInfo: {},
        showTableSelect: false
      }
    }
  },
  computed: {
    dataRule () {
      const validateTable = (rule, value, callback) => {
        if (!this.dataForm.tableInfo.tableName) {
          return callback(new Error('必填项不能为空'))
        }
        callback()
      }
      return {
        datasourceId: [
          { required: true, message: '必填项不能为空', trigger: 'blur' }
        ],
        tableInfo: [
          { validator: validateTable, trigger: 'blur' }
        ]
      }
    }
  },
  methods: {
    init () {
      this.visible = true
      this.$nextTick(() => {
        this.$refs['dataForm'].resetFields()
        this.getDataSource()
      })
    },
    // 获取数据源
    getDataSource () {
      this.$http.get('/devtools/datasource/list').then(({ data: res }) => {
        if (res.code !== 0) {
          return this.$message.error(res.msg)
        }
        this.dataForm.datasourceList = res.data
      }).catch(() => {})
    },
    // 获取数据表
    getTableInfoList () {
      this.dataForm.showTableSelect = false
      this.dataForm.tableInfo = {}

      this.$http.get('/devtools/datasource/table/list/' + this.dataForm.datasourceId).then(({ data: res }) => {
        if (res.code !== 0) {
          return this.$message.error(res.msg)
        }
        this.dataForm.tableInfoList = res.data
        this.dataForm.showTableSelect = true
      }).catch(() => {})
    },
    // 表单提交
    dataFormSubmitHandle: debounce(function () {
      this.$refs['dataForm'].validate((valid) => {
        if (!valid) {
          return false
        }
        this.$http[!this.dataForm.id ? 'post' : 'put']('/devtools/datasource/table', this.dataForm.tableInfo).then(({ data: res }) => {
          if (res.code !== 0) {
            return this.$message.error(res.msg)
          }
          this.$message({
            message: this.$t('prompt.success'),
            type: 'success',
            duration: 500,
            onClose: () => {
              this.visible = false
              this.$emit('refreshDataList')
            }
          })
        }).catch(() => {})
      })
    }, 1000, { 'leading': true, 'trailing': false })
  }
}
</script>