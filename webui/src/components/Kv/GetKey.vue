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

      <b-button @click="refreshValue" class="my-2" variant="primary" :disabled="loading || value === ''">Load</b-button>

      <Promised :promise="valuePromise" class="mt-2">
        <template v-slot:pending>
          <div class="text-center">
            <Loader />
          </div>
        </template>
        <template v-slot="data">
          <b-form-group label="Value" label-for="value">
            <pre>{{ data }}</pre>
          </b-form-group>
        </template>
        <template v-slot:rejected="error">
          <b-alert show variant="danger">{{ error.message }}</b-alert>
        </template>
      </Promised>
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
    // Lucid key - value because `v-model` usage
    value: {
      type: String,
      required: true
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
        this.valuePromise = lucidApi.getKey(this.value).then(res => res.json())
        await this.valuePromise
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
