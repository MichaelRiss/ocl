// use std::mem;
// use std::ptr;

use cl_h::{ self, cl_command_queue, cl_context, cl_device_id };
use super::{ Context };

// [FIXME] TODO: Implement a constructor which accepts a cl_device_id.
#[derive(Clone)]
pub struct Queue {
	obj: cl_command_queue,
	context_obj: cl_context,
	device_id: cl_device_id,
}

impl Queue {
	pub fn new(context: &Context, device_idx: Option<usize>) -> Queue {
		let device_id = context.resolve_device_id(device_idx);
		let obj: cl_command_queue = super::create_command_queue(context.obj(), device_id); 

		Queue {
			obj: obj,
			context_obj: context.obj(),
			device_id: device_id,			
		}
	}	

	pub fn finish(&self) {
		super::finish(self.obj);
	}

	pub fn obj(&self) -> cl_command_queue {
		self.obj
	}

	pub fn context_obj(&self) -> cl_context {
		self.context_obj
	}

	pub fn device_id(&self) -> cl_device_id {
		self.device_id
	}

	// Note: Do not move this to a Drop impl in case this Queue has been cloned.
	pub fn release(&mut self) {
		unsafe {
			cl_h::clReleaseCommandQueue(self.obj);
		}
	}
}
