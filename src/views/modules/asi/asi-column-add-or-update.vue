<template>
  <el-dialog :visible.sync="visible" :title="(!dataForm.id ? $t('add') : $t('update'))+'-'+curarrGroup.name" :close-on-click-modal="false"
             :close-on-press-escape="false">
    <el-form :model="dataForm" :rules="dataRule" ref="dataForm" @keyup.enter.native="dataFormSubmitHandle()"
             label-width="120px">
      <el-form-item prop="column_name" label="列名称">
        <el-input v-model="dataForm.column_name" placeholder="列名称"></el-input>
      </el-form-item>
      <el-form-item prop="column_code" label="列编码">
        <el-input v-model="dataForm.column_code" placeholder="列编码"></el-input>
      </el-form-item>
      <el-form-item prop="data_type" label="数组类型">
        <el-select v-model="dataForm.data_type" placeholder="数据类型" clearable>
          <el-option v-for="data in dataTypes" :key="data.dict_value" :label="data.dict_label" :value="data.dict_value">
            {{ data.dict_label }}
          </el-option>
        </el-select>
      </el-form-item>
      <el-form-item prop="data_type" label="数组类型">
        <el-radio-group v-model="dataForm.is_required">
          <el-radio label="Y">必填</el-radio>
          <el-radio label="N">可空</el-radio>
        </el-radio-group>
      </el-form-item>
      <el-form-item prop="display_order" label="最大长度">
        <el-input-number v-model="dataForm.max_length" controls-position="right" :min="1"
                         label="最大长度"></el-input-number>
      </el-form-item>
      <el-form-item prop="display_order" label="排序">
        <el-input-number v-model="dataForm.display_order" controls-position="right" :min="0"
                         :label="$t('dept.sort')"></el-input-number>
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
import { getDictDataList } from '@/utils'

export default {
  data () {
    return {
      visible: false,
      groupListVisible: false,
      dataTypes: [],
      curarrGroup: {},
      dataForm: {
        id: '',
        column_name: '',
        group_code: '',
        column_code: '',
        data_type: '',
        max_length: 10,
        is_required: 'Y',
        example_value: '',
        display_order: 0
      }
    }
  },
  computed: {
    dataRule () {
      return {}
    }
  },
  methods: {
    init (curarrGroup) {
      this.visible = true
      this.curarrGroup = curarrGroup
      this.$nextTick(() => {
        this.$refs['dataForm'].resetFields()
        // eslint-disable-next-line no-undef
        this.dataTypes = getDictDataList('asi_colums_type')
        if (this.dataForm.id) {
          this.getInfo()
        }
      })
    },
    // 获取信息
    getInfo () {
      this.$http.get(`/asi/column/get_column_one/${this.dataForm.id}`).then(({ data: res }) => {
        // eslint-disable-next-line eqeqeq
        if (res.code != 0) {
          return this.$message.error(res.msg)
        }
        this.dataForm = {
          ...this.dataForm,
          ...res.data
        }
      }).catch(() => {
      })
    },
    // 表单提交
    dataFormSubmitHandle: debounce(function () {
      this.$refs['dataForm'].validate((valid) => {
        if (!valid) {
          return false
        }

        this.$http[!this.dataForm.id ? 'post' : 'put']('/asi/column/list/' + this.curarrGroup.group_code, this.dataForm).then(({ data: res }) => {
          // eslint-disable-next-line eqeqeq
          if (res.code != 0) {
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
        }).catch(() => {
        })
      })
    }, 1000, { 'leading': true, 'trailing': false })
  }
}
</script>
