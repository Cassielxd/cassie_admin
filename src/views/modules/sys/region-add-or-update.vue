<template>
  <el-dialog :visible.sync="visible" :title="insert ? $t('add') : $t('update')" :close-on-click-modal="false" :close-on-press-escape="false">
    <el-form :model="dataForm" :rules="dataRule" ref="dataForm" @keyup.enter.native="dataFormSubmitHandle()" label-width="120px">
      <el-form-item prop="parentName" :label="$t('region.parentName')">
        <ren-region-tree v-model="dataForm.pid" :placeholder="$t('region.select')" :parent-name.sync="dataForm.parentName"></ren-region-tree>
      </el-form-item>
      <el-form-item prop="name" :label="$t('region.name')">
        <el-input v-model="dataForm.name" :placeholder="$t('region.name')"></el-input>
      </el-form-item>
      <el-form-item prop="id" :label="$t('region.id')">
        <el-input v-model="dataForm.id" :disabled="!insert" :placeholder="$t('region.id')"></el-input>
      </el-form-item>
      <el-form-item prop="sort" :label="$t('region.sort')">
        <el-input-number v-model="dataForm.sort" controls-position="right" :min="0" :label="$t('region.sort')"></el-input-number>
      </el-form-item>
    </el-form>
    <template slot="footer">
      <el-button @click="visible = false">{{ $t('cancel') }}</el-button>
      <el-button type="primary" @click="dataFormSubmitHandle()">{{ $t('confirm') }}</el-button>
    </template>
  </el-dialog>
</template>

<script>
import debounce from 'lodash/debounce'
export default {
  data () {
    return {
      visible: false,
      insert: true,
      regionList: [],
      regionListVisible: false,
      dataForm: {
        id: '',
        name: '',
        pid: '0',
        parentName: '',
        sort: 0
      }
    }
  },
  computed: {
    dataRule () {
      return {
        id: [
          { required: true, message: this.$t('validate.required'), trigger: 'blur' }
        ],
        name: [
          { required: true, message: this.$t('validate.required'), trigger: 'blur' }
        ],
        sort: [
          { required: true, message: this.$t('validate.required'), trigger: 'blur' }
        ]
      }
    }
  },
  methods: {
    init (id) {
      this.insert = true
      this.visible = true
      this.dataForm.pid = '0'
      this.$nextTick(() => {
        this.$refs['dataForm'].resetFields()
        this.dataForm.id = id
        if (this.dataForm.id) {
          this.insert = false
          this.getInfo()
        }
      })
    },
    // 获取信息
    getInfo () {
      this.$http.get(`/sys/region/${this.dataForm.id}`).then(({ data: res }) => {
        if (res.code !== 0) {
          return this.$message.error(res.msg)
        }
        this.dataForm = {
          ...this.dataForm,
          ...res.data
        }
        this.$refs.regionListTree.setCurrentKey(this.dataForm.pid)
      }).catch(() => {})
    },
    // 表单提交
    dataFormSubmitHandle: debounce(function () {
      this.$refs['dataForm'].validate((valid) => {
        if (!valid) {
          return false
        }
        this.$http[this.insert ? 'post' : 'put']('/sys/region', this.dataForm).then(({ data: res }) => {
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

<style lang="scss">
.mod-sys__region {
  .region-list {
    .el-input__inner,
    .el-input__suffix {
      cursor: pointer;
    }
  }
}
</style>
