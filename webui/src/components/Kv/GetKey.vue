<template>
  <div>
    <b-card header="Targetted key">
      <b-form-group label="Key" label-for="key">
        <b-form-input
          id="key"
          :value="value"
          @input="$emit('input', $event)"
          placeholder="Key"
          :disabled="loading"
        />
      </b-form-group>

      <Promised :promise="valuePromise" class="mt-2">
        <template v-slot:pending>
          <div class="text-center">
            <Loader />
          </div>
        </template>
        <template v-slot="data">
          <b-form-group label="Value" label-for="value">
            <b-form-textarea
              id="value"
              :value="data"
              placeholder="Value"
              readonly
            />
          </b-form-group>
        </template>
        <template v-slot:rejected="error">
          <b-alert show variant="danger">{{ error.message }}</b-alert>
        </template>
      </Promised>

      <b-button @click="refreshValue" variant="primary" :disabled="loading || value === ''">Refresh</b-button>
    </b-card>
  </div>
</template>

<script>
import Loader from '@/components/Loader'

import { lucidApi } from '@/lucidApi'

export default {
  components: {
    Loader
  },
  props: {
    value: {
      type: String
    }
  },
  data() {
    return {
      valuePromise: null,
      loading: false,
      error: null
    }
  },
  methods: {
    async refreshValue() {
      this.loading = true
      this.error = null
      try {
        this.valuePromise = lucidApi.getKey(this.value) .then(res => res.json())
      }
      catch (error) {
        this.error = error.message
      }
      finally {
        this.loading = false
      }
    }
  }
}
</script>
