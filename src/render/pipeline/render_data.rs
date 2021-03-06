use super::structs::RaytraceUniformData;
use crate::game::Game;
use crate::render::constants::*;
use crate::render::general::command_buffer::CommandBuffer;
use crate::render::general::core::Core;
use crate::render::general::structures::{
    Buffer, BufferWrapper, DataDestination, ExtentWrapper, ImageOptions, ImageWrapper,
    SampledImage, SamplerOptions, StorageImage,
};
use crate::util::{self, prelude::*};
use crate::world::ChunkStorage;
use ash::vk;
use std::rc::Rc;

pub struct RenderData {
    pub core: Rc<Core>,

    pub material_image: SampledImage,
    pub minefield_image: SampledImage,

    pub lighting_buffer: StorageImage,
    pub completed_buffer: StorageImage,
    pub depth_buffer: StorageImage,
    pub normal_buffer: StorageImage,

    pub lighting_pong_buffer: StorageImage,
    pub albedo_buffer: StorageImage,
    pub emission_buffer: StorageImage,
    pub fog_color_buffer: StorageImage,

    pub blue_noise: SampledImage,

    pub raytrace_uniform_data: RaytraceUniformData,
    pub raytrace_uniform_data_buffer: Buffer<RaytraceUniformData>,
}

impl RenderData {
    fn create_framebuffer(core: Rc<Core>, name: &str, format: vk::Format) -> StorageImage {
        let dimensions = core.swapchain.swapchain_extent;
        let options = ImageOptions {
            typ: vk::ImageType::TYPE_2D,
            extent: vk::Extent3D {
                width: dimensions.width,
                height: dimensions.height,
                depth: 1,
            },
            format,
            usage: vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::STORAGE,
            ..Default::default()
        };
        StorageImage::create(core, name, &options)
    }

    fn create_material_image(core: Rc<Core>) -> SampledImage {
        let image_options = ImageOptions {
            typ: vk::ImageType::TYPE_3D,
            extent: vk::Extent3D {
                width: ROOT_BLOCK_SIZE as u32,
                height: ROOT_BLOCK_SIZE as u32,
                depth: ROOT_BLOCK_SIZE as u32,
            },
            format: vk::Format::R32_UINT,
            usage: vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED,
            ..Default::default()
        };
        let sampler_options = SamplerOptions {
            min_filter: vk::Filter::NEAREST,
            mag_filter: vk::Filter::NEAREST,
            address_mode: vk::SamplerAddressMode::CLAMP_TO_BORDER,
            border_color: vk::BorderColor::INT_OPAQUE_BLACK,
            unnormalized_coordinates: false,
            mipmap_mode: vk::SamplerMipmapMode::NEAREST,
        };
        SampledImage::create(
            core.clone(),
            "material_img",
            &image_options,
            &sampler_options,
        )
    }

    fn create_minefield(core: Rc<Core>) -> SampledImage {
        let image_options = ImageOptions {
            typ: vk::ImageType::TYPE_3D,
            extent: vk::Extent3D {
                width: ROOT_BLOCK_SIZE as u32,
                height: ROOT_BLOCK_SIZE as u32,
                depth: ROOT_BLOCK_SIZE as u32,
            },
            format: vk::Format::R8_UINT,
            usage: vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED,
            ..Default::default()
        };
        let sampler_options = SamplerOptions {
            min_filter: vk::Filter::NEAREST,
            mag_filter: vk::Filter::NEAREST,
            address_mode: vk::SamplerAddressMode::CLAMP_TO_BORDER,
            border_color: vk::BorderColor::INT_OPAQUE_BLACK,
            unnormalized_coordinates: true,
            ..Default::default()
        };
        SampledImage::create(
            core.clone(),
            "minefield_img",
            &image_options,
            &sampler_options,
        )
    }

    fn create_blue_noise(core: Rc<Core>) -> SampledImage {
        let image_options = ImageOptions {
            typ: vk::ImageType::TYPE_2D,
            extent: vk::Extent3D {
                width: BLUE_NOISE_WIDTH as u32,
                height: BLUE_NOISE_HEIGHT as u32,
                depth: 1,
            },
            format: vk::Format::R8G8B8A8_UNORM,
            usage: vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED,
            ..Default::default()
        };
        let sampler_options = SamplerOptions {
            min_filter: vk::Filter::NEAREST,
            mag_filter: vk::Filter::NEAREST,
            address_mode: vk::SamplerAddressMode::CLAMP_TO_EDGE,
            unnormalized_coordinates: true,
            ..Default::default()
        };
        let tex =
            SampledImage::create(core.clone(), "blue_noise", &image_options, &sampler_options);
        tex.load_from_png_rgba8(include_bytes!("blue_noise_512.png"));
        tex
    }

    fn create_raytrace_uniform_data() -> RaytraceUniformData {
        RaytraceUniformData {
            sun_angle: 0.0,
            seed: 0,
            origin: [0.0, 0.0, 0.0].into(),
            forward: [0.0, 0.0, 0.0].into(),
            up: [0.0, 0.0, 0.0].into(),
            right: [0.0, 0.0, 0.0].into(),
            old_origin: [0.0, 0.0, 0.0].into(),
            old_transform_c0: [0.0, 0.0, 0.0].into(),
            old_transform_c1: [0.0, 0.0, 0.0].into(),
            old_transform_c2: [0.0, 0.0, 0.0].into(),
            region_offset: [0, 0, 0].into(),
            rotation: [-64, -64, 0].into(),
            space_offset: [-64, -64, 0].into(),
            _padding0: 0,
            _padding1: 0,
            _padding2: 0,
            _padding3: 0,
            _padding4: 0,
            _padding5: 0,
            _padding6: 0,
            _padding7: 0,
            _padding8: 0,
            _padding9: 0,
            _padding10: 0,
            _padding11: 0,
        }
    }

    pub fn create(core: Rc<Core>) -> RenderData {
        let rgba16_unorm = vk::Format::R16G16B16A16_UNORM;
        let rgba8_unorm = vk::Format::R8G8B8A8_UNORM;
        let r16_uint = vk::Format::R16_UINT;
        let r8_uint = vk::Format::R8_UINT;

        RenderData {
            core: core.clone(),

            material_image: Self::create_material_image(core.clone()),
            minefield_image: Self::create_minefield(core.clone()),

            lighting_buffer: Self::create_framebuffer(core.clone(), "lighting_buf", rgba16_unorm),
            completed_buffer: Self::create_framebuffer(core.clone(), "completed_buf", rgba16_unorm),
            depth_buffer: Self::create_framebuffer(core.clone(), "depth_buf", r16_uint),
            normal_buffer: Self::create_framebuffer(core.clone(), "normal_buf", r8_uint),

            lighting_pong_buffer: Self::create_framebuffer(
                core.clone(),
                "lighting_pong_buf",
                rgba16_unorm,
            ),
            albedo_buffer: Self::create_framebuffer(core.clone(), "albedo_buf", rgba8_unorm),
            emission_buffer: Self::create_framebuffer(core.clone(), "emission_buf", rgba8_unorm),
            fog_color_buffer: Self::create_framebuffer(core.clone(), "fog_color_buf", rgba8_unorm),

            blue_noise: Self::create_blue_noise(core.clone()),

            raytrace_uniform_data: Self::create_raytrace_uniform_data(),
            raytrace_uniform_data_buffer: Buffer::create(
                core.clone(),
                "raytrace_uniform_data",
                1,
                vk::BufferUsageFlags::UNIFORM_BUFFER,
            ),
        }
    }

    fn make_world_upload_buffers(&mut self, world: &mut ChunkStorage) -> (Buffer<u32>, Buffer<u8>) {
        let mut material_buffer = Buffer::create(
            self.core.clone(),
            "material_buf",
            ROOT_BLOCK_VOLUME as u64,
            vk::BufferUsageFlags::TRANSFER_SRC,
        );
        let mut minefield_buffer = Buffer::create(
            self.core.clone(),
            "minefield_buf",
            ROOT_BLOCK_VOLUME as u64,
            vk::BufferUsageFlags::TRANSFER_SRC,
        );

        let mut material_buffer_data = material_buffer.bind_all();
        let mut minefield_buffer_data = minefield_buffer.bind_all();
        let mut gen_time = 0;
        let mut copy_time = 0;
        for chunk_coord in util::coord_iter_3d(ROOT_CHUNK_SIZE) {
            let world_coord = chunk_coord.signed().sub((
                (ROOT_CHUNK_SIZE as isize / 2),
                (ROOT_CHUNK_SIZE as isize / 2),
                (ROOT_CHUNK_SIZE as isize / 2),
            ));
            let timer = std::time::Instant::now();
            let chunk =
                world.borrow_packed_chunk_data(&(world_coord.0, world_coord.1, world_coord.2));
            gen_time += timer.elapsed().as_millis();
            let timer = std::time::Instant::now();
            chunk.copy_materials(
                util::scale_coord_3d(&chunk_coord, CHUNK_SIZE).signed(),
                material_buffer_data.as_slice_mut(),
                ROOT_BLOCK_SIZE,
            );
            chunk.copy_minefield(
                util::scale_coord_3d(&chunk_coord, CHUNK_SIZE).signed(),
                minefield_buffer_data.as_slice_mut(),
                ROOT_BLOCK_SIZE,
            );
            copy_time += timer.elapsed().as_millis();
        }
        println!("Gen time: {}ms, copy time: {}ms", gen_time, copy_time);
        drop(material_buffer_data);
        drop(minefield_buffer_data);

        (material_buffer, minefield_buffer)
    }

    fn upload_buf_commands(
        commands: &mut CommandBuffer,
        buffer: &impl BufferWrapper,
        image: &(impl ImageWrapper + ExtentWrapper),
    ) {
        commands.transition_layout(
            image,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        );
        commands.copy_buffer_to_image(buffer, image, image);
        commands.transition_layout(
            image,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::ImageLayout::GENERAL,
        );
    }

    pub fn initialize(&mut self, game: &mut Game) {
        let world = game.borrow_world_mut();
        let (material_buffer, minefield_buffer) = self.make_world_upload_buffers(world);

        let mut commands = CommandBuffer::create_single(self.core.clone());
        commands.begin_one_time_submit();
        Self::upload_buf_commands(&mut commands, &material_buffer, &self.material_image);
        Self::upload_buf_commands(&mut commands, &minefield_buffer, &self.minefield_image);
        let generic_layout_images = [
            &self.albedo_buffer,
            &self.completed_buffer,
            &self.depth_buffer,
            &self.emission_buffer,
            &self.fog_color_buffer,
            &self.lighting_buffer,
            &self.lighting_pong_buffer,
            &self.normal_buffer,
        ];
        for image in generic_layout_images.iter() {
            commands.transition_layout(
                *image,
                vk::ImageLayout::UNDEFINED,
                vk::ImageLayout::GENERAL,
            );
        }
        commands.transition_layout(
            &self.blue_noise,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        );
        commands.end();
        commands.blocking_execute_and_destroy();
    }
}
