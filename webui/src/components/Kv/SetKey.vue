<template>
  <div>
    <b-card header="Value to set">
      <Promised :promise="valuePromise" v-slot:combined="{ isPending, data, error }">
        <b-form @submit.prevent="setValue" class="mb-3">
          <b-form-group label="New value" label-for="value">
            <b-form-textarea
              v-model="value"
              id="value"
              placeholder="New value"
              :disabled="isKvLoading || isPending"
            />
          </b-form-group>

          <b-button type="submit" variant="primary" :disabled="isKvLoading || isPending || value === ''">Load</b-button>
        </b-form>

        <div v-if="isPending" class="text-center">
          <Loader />
        </div>
        <b-alert v-else-if="data" show variant="success" v-text="data" />
        <b-alert v-else-if="error" show variant="danger" v-text="error" />
      </Promised>
    </b-card>
  </div>
</template>

<script>
import { mapMutations, mapGetters } from 'vuex'

import Loader from '@/components/Loader'
import { Lucid } from '@/lucidApi'

export default {
  components: {
    Loader
  },
  props: {
    lucidKey: {
      type: String,
      required: true
    }
  },
  data() {
    return {
      value: '',
      valuePromise: Promise.resolve()
    }
  },
  computed: {
    ...mapGetters(['isKvLoading'])
  },
  methods: {
    ...mapMutations(['setKvLoading']),
    async setValue() {
      this.setKvLoading(true)
      try {
        this.valuePromise = Lucid.storeKeyDataJson(this.lucidKey, this.value)
          .then(res => res.json())
          .then(res => res.message)
        await this.valuePromise
      }
      catch (error) {
        this.valuePromise = Promise.reject(error.message)
      }
      finally {
        this.setKvLoading(false)
      }
    }
  }
}
</script>

<style scoped>
textarea {
  width: 100%;
  height: 500px;
}
</style>
