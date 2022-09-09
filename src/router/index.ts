import { createRouter, createMemoryHistory } from "vue-router";
import HomeLayout from "@/components/HomeLayout.vue";
import ChooseImage from "@/components/steps/ChooseImage.vue";
import StorageSelect from "@/components/select/StorageSelect.vue";
import FlashImage from "@/components/steps/FlashImage.vue";

const router = createRouter({
  history: createMemoryHistory(),
  routes: [
    {
      path: "/",
      components: {
        ChooseImage: ChooseImage,
        default: HomeLayout,
      },
      children: [
        {
          path: "",
          name: "select-image",
          components: {
            ChooseImage: ChooseImage,
            StorageSelect: StorageSelect,
            FlashImage: FlashImage,
          },
        },
        {
          path: "select-storage",
          name: "select-storage",
          components: {
            ChooseImage: ChooseImage,
            StorageSelect: StorageSelect,
            FlashImage: FlashImage,
          },
        },
        {
          path: "flash-image",
          name: "flash-image",
          components: {
            ChooseImage: ChooseImage,
            StorageSelect: StorageSelect,
            FlashImage: FlashImage,
          },
        },
      ],
    },
  ],
});

export default router;
