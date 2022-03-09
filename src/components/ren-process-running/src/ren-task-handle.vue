<template>
  <el-dialog :visible.sync="visible" :title="handleTitle" :close-on-click-modal="false" :close-on-press-escape="false">
    <el-form :model="dataForm" :rules="dataRule" ref="dataForm" @keyup.enter.native="dataFormSubmitHandle()" :label-width="$i18n.locale === 'en-US' ? '120px' : '80px'">
      <el-form-item :label="$t('process.comment')" prop="comment">
        <el-input type="textarea" v-model="dataForm.comment" :placeholder="$t('process.comment')"></el-input>
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
import qs from 'qs'
import mixinViewModule from '@/mixins/view-module'
export default {
  mixins: [mixinViewModule],
  data () {
    return {
      mixinViewModuleOptions: {
        createdIsNeed: false
      },
      visible: false,
      handleType: '',
      handleTitle: '',
      dataForm: {
        comment: '',
        taskId: ''
      }
    }
  },
  computed: {
    dataRule () {
      return {
        comment: [
          { required: true, message: this.$t('validate.required'), trigger: 'blur' }
        ]
      }
    }
  },
  methods: {
    init () {
      this.visible = true
      if (this.handleType === 'complete') {
        this.handleTitle = this.$t('process.completeTask')
      } else if (this.handleType === 'reject') {
        this.handleTitle = this.$t('process.rejectTask')
      }
      this.$nextTick(() => {
        this.$refs['dataForm'].resetFields()
      })
    },
    // 表单提交
    dataFormSubmitHandle () {
      if (this.handleType === 'complete') {
        this.completeTask()
      } else if (this.handleType === 'reject') {
        this.rejectTask()
      }
    },
    // 驳回
    rejectTask: debounce(function () {
      this.$refs['dataForm'].validate((valid) => {
        if (!valid) {
          return false
        }
        if (!this.dataForm.taskId) {
          return false
        }
        const params = qs.stringify({
          'taskId': this.dataForm.taskId,
          'comment': this.dataForm.comment
        });
        this.$http['post']('/flow/common/reject?', params).then(({ data: res }) => {
          if (res.code !== 0) {
            this.$message.error(res.msg)
            return
          }
          this.$message({
            message: this.$t('prompt.success'),
            type: 'success',
            duration: 500,
            onClose: () => {
              this.visible = false
              this.closeCurrentTab()
            }
          })
        })
      })
    }, 1000, { 'leading': true, 'trailing': false }),
    // 通过
    completeTask: debounce(function () {
      this.$refs['dataForm'].validate((valid) => {
        if (!valid) {
          return false
        }
        if (!this.dataForm.taskId) {
          return false
        }
        const params = qs.stringify({
          'taskId': this.dataForm.taskId,
          'comment': this.dataForm.comment
        })
        this.$http.post('/flow/common/complete?' + params).then(({ data: res }) => {
          if (res.code !== 0) {
            this.$message.error(res.msg)
            return
          }
          this.$message({
            message: this.$t('prompt.success'),
            type: 'success',
            duration: 500,
            onClose: () => {
              this.visible = false
              this.closeCurrentTab()
            }
          })
        }).catch(() => {})
      })
    }, 1000, { 'leading': true, 'trailing': false })
  }
}
</script>
