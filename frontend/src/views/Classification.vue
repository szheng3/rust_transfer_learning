<script setup>
import {ref} from "vue";
import axios from "axios";
import {ContentLoader} from "vue-content-loader";

const selectedFile = ref(null);
const result = ref({status: 'success', message: ''});
const loading = ref(false);
const showResult = ref(false);

const preview = ref(null)

const onFileInputChange = (event) => {
  selectedFile.value = event.target.files[0];
  const reader = new FileReader()
  reader.readAsDataURL(event.target.files[0])
  reader.onload = () => {
    preview.value = reader.result
  }

}

const upload = async () => {
  loading.value = true
  const formData = new FormData();
  formData.append('image', selectedFile.value);

  try {
    const response = await fetch('/api/upload', {
      method: 'POST',
      body: formData,
    });
    const val = await response.json();
    showResult.value = true;
    result.value = val

  } catch (error) {
    console.error(error);

  } finally {
    loading.value = false;
  }
}
</script>
<template>
  <v-container>
    <v-row >
      <v-col cols="12" sm="6" >
        <v-sheet rounded >
          <v-form @submit.prevent="" class="pa-3">
            <v-file-input
                accept="image/*"
                label="Upload an image"
                prepend-icon="mdi-camera"
                @change="onFileInputChange"
                outlined
            ></v-file-input>
            <v-img
                max-height="500"
                v-if="preview"
                :src="preview"
                contain
                class="my-3"
            ></v-img>
            <ContentLoader v-else viewBox="0 0 400 350">
              <rect x="0" y="13" rx="4" ry="4" width="400" height="9"/>
              <rect x="0" y="29" rx="4" ry="4" width="100" height="8"/>
              <rect x="0" y="50" rx="4" ry="4" width="400" height="10"/>
              <rect x="0" y="65" rx="4" ry="4" width="400" height="10"/>
              <rect x="0" y="79" rx="4" ry="4" width="100" height="10"/>
              <rect x="0" y="95" rx="4" ry="4" width="400" height="10"/>
              <rect x="0" y="110" rx="4" ry="4" width="400" height="10"/>
              <rect x="0" y="125" rx="4" ry="4" width="100" height="10"/>
              <rect x="0" y="140" rx="5" ry="5" width="400" height="200"/>
            </ContentLoader>
            <v-btn
                :loading="loading"
                type="submit"
                color="primary"
                @click="upload"
                class="mt-3"
                block
            >
              Submit
            </v-btn>
          </v-form>
        </v-sheet>
      </v-col>
      <v-col cols="12" sm="6">
        <v-card class="pa-4" v-if="showResult" variant="flat">
          <v-card-title class="headline">Classification Result</v-card-title>
          <v-card-text>{{ result.message }}</v-card-text>
        </v-card>
        <v-card v-if="!(showResult )" variant="flat">

          <v-card-text>
            <ContentLoader viewBox="0 0 400 350">
              <rect x="0" y="13" rx="4" ry="4" width="400" height="9"/>
              <rect x="0" y="29" rx="4" ry="4" width="100" height="8"/>
              <rect x="0" y="50" rx="4" ry="4" width="400" height="10"/>
              <rect x="0" y="65" rx="4" ry="4" width="400" height="10"/>
              <rect x="0" y="79" rx="4" ry="4" width="100" height="10"/>
              <rect x="0" y="95" rx="4" ry="4" width="400" height="10"/>
              <rect x="0" y="110" rx="4" ry="4" width="400" height="10"/>
              <rect x="0" y="125" rx="4" ry="4" width="100" height="10"/>
              <rect x="0" y="140" rx="5" ry="5" width="400" height="200"/>

            </ContentLoader>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<style>

.headline {
  font-size: 1.5rem;
  font-weight: 600;
  margin-bottom: 1rem;
}
</style>