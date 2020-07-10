//! AVM1 object type to represent Sound objects.

use crate::avm1::activation::Activation;
use crate::avm1::error::Error;
use crate::avm1::function::Executable;
use crate::avm1::property::Attribute;
use crate::avm1::{Object, ObjectPtr, ScriptObject, TObject, Value};
use crate::backend::audio::{SoundHandle, SoundInstanceHandle};
use crate::context::UpdateContext;
use crate::display_object::DisplayObject;
use enumset::EnumSet;
use gc_arena::{Collect, GcCell, MutationContext};
use std::borrow::Cow;
use std::fmt;

/// A SounObject that is tied to a sound from the AudioBackend.
#[derive(Clone, Copy, Collect)]
#[collect(no_drop)]
pub struct SoundObject<'gc>(GcCell<'gc, SoundObjectData<'gc>>);

pub struct SoundObjectData<'gc> {
    /// The underlying script object.
    ///
    /// This is used to handle "expando properties" on AVM1 display nodes, as
    /// well as the underlying prototype chain.
    base: ScriptObject<'gc>,

    /// The sound that is attached to this object.
    sound: Option<SoundHandle>,

    /// The instance of the last played sound on this object.
    sound_instance: Option<SoundInstanceHandle>,

    /// Sounds in AVM1 are tied to a speicifc movie clip.
    owner: Option<DisplayObject<'gc>>,

    /// Position of the last playing sound in milliseconds.
    position: u32,

    /// Duration of the currently attached sound in milliseconds.
    duration: u32,
}

unsafe impl<'gc> Collect for SoundObjectData<'gc> {
    fn trace(&self, cc: gc_arena::CollectionContext) {
        self.base.trace(cc);
        self.owner.trace(cc);
    }
}

impl fmt::Debug for SoundObject<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let this = self.0.read();
        f.debug_struct("SoundObject")
            .field("sound", &this.sound)
            .field("sound_instance", &this.sound_instance)
            .field("owner", &this.owner)
            .finish()
    }
}

impl<'gc> SoundObject<'gc> {
    pub fn empty_sound(
        gc_context: MutationContext<'gc, '_>,
        proto: Option<Object<'gc>>,
    ) -> SoundObject<'gc> {
        SoundObject(GcCell::allocate(
            gc_context,
            SoundObjectData {
                base: ScriptObject::object(gc_context, proto),
                sound: None,
                sound_instance: None,
                owner: None,
                position: 0,
                duration: 0,
            },
        ))
    }

    pub fn duration(self) -> u32 {
        self.0.read().duration
    }

    pub fn set_duration(self, gc_context: MutationContext<'gc, '_>, duration: u32) {
        self.0.write(gc_context).duration = duration;
    }

    pub fn sound(self) -> Option<SoundHandle> {
        self.0.read().sound
    }

    pub fn set_sound(self, gc_context: MutationContext<'gc, '_>, sound: Option<SoundHandle>) {
        self.0.write(gc_context).sound = sound;
    }

    pub fn sound_instance(self) -> Option<SoundInstanceHandle> {
        self.0.read().sound_instance
    }

    pub fn set_sound_instance(
        self,
        gc_context: MutationContext<'gc, '_>,
        sound_instance: Option<SoundInstanceHandle>,
    ) {
        self.0.write(gc_context).sound_instance = sound_instance;
    }

    pub fn owner(self) -> Option<DisplayObject<'gc>> {
        self.0.read().owner
    }

    pub fn set_owner(
        self,
        gc_context: MutationContext<'gc, '_>,
        owner: Option<DisplayObject<'gc>>,
    ) {
        self.0.write(gc_context).owner = owner;
    }

    pub fn position(self) -> u32 {
        self.0.read().position
    }

    pub fn set_position(self, gc_context: MutationContext<'gc, '_>, position: u32) {
        self.0.write(gc_context).position = position;
    }

    fn base(self) -> ScriptObject<'gc> {
        self.0.read().base
    }
}

impl<'gc> TObject<'gc> for SoundObject<'gc> {
    fn get_local(
        &self,
        name: &str,
        activation: &mut Activation<'_, 'gc>,
        context: &mut UpdateContext<'_, 'gc, '_>,
        this: Object<'gc>,
    ) -> Result<Value<'gc>, Error<'gc>> {
        self.base().get_local(name, activation, context, this)
    }

    fn set(
        &self,
        name: &str,
        value: Value<'gc>,
        activation: &mut Activation<'_, 'gc>,
        context: &mut UpdateContext<'_, 'gc, '_>,
    ) -> Result<(), Error<'gc>> {
        self.base().set(name, value, activation, context)
    }
    fn call(
        &self,
        name: &str,
        activation: &mut Activation<'_, 'gc>,
        context: &mut UpdateContext<'_, 'gc, '_>,
        this: Object<'gc>,
        base_proto: Option<Object<'gc>>,
        args: &[Value<'gc>],
    ) -> Result<Value<'gc>, Error<'gc>> {
        self.base()
            .call(name, activation, context, this, base_proto, args)
    }

    fn call_setter(
        &self,
        name: &str,
        value: Value<'gc>,
        activation: &mut Activation<'_, 'gc>,
        context: &mut UpdateContext<'_, 'gc, '_>,
    ) -> Option<Executable<'gc>> {
        self.base().call_setter(name, value, activation, context)
    }

    #[allow(clippy::new_ret_no_self)]
    fn new(
        &self,
        activation: &mut Activation<'_, 'gc>,
        context: &mut UpdateContext<'_, 'gc, '_>,
        _this: Object<'gc>,
        _args: &[Value<'gc>],
    ) -> Result<Object<'gc>, Error<'gc>> {
        Ok(
            SoundObject::empty_sound(context.gc_context, Some(activation.avm.prototypes.sound))
                .into(),
        )
    }

    fn delete(
        &self,
        activation: &mut Activation<'_, 'gc>,
        gc_context: MutationContext<'gc, '_>,
        name: &str,
    ) -> bool {
        self.base().delete(activation, gc_context, name)
    }

    fn proto(&self) -> Option<Object<'gc>> {
        self.base().proto()
    }

    fn set_proto(&self, gc_context: MutationContext<'gc, '_>, prototype: Option<Object<'gc>>) {
        self.base().set_proto(gc_context, prototype);
    }

    fn define_value(
        &self,
        gc_context: MutationContext<'gc, '_>,
        name: &str,
        value: Value<'gc>,
        attributes: EnumSet<Attribute>,
    ) {
        self.base()
            .define_value(gc_context, name, value, attributes)
    }

    fn set_attributes(
        &mut self,
        gc_context: MutationContext<'gc, '_>,
        name: Option<&str>,
        set_attributes: EnumSet<Attribute>,
        clear_attributes: EnumSet<Attribute>,
    ) {
        self.base()
            .set_attributes(gc_context, name, set_attributes, clear_attributes)
    }

    fn add_property(
        &self,
        gc_context: MutationContext<'gc, '_>,
        name: &str,
        get: Executable<'gc>,
        set: Option<Executable<'gc>>,
        attributes: EnumSet<Attribute>,
    ) {
        self.base()
            .add_property(gc_context, name, get, set, attributes)
    }

    fn add_property_with_case(
        &self,
        activation: &mut Activation<'_, 'gc>,
        gc_context: MutationContext<'gc, '_>,
        name: &str,
        get: Executable<'gc>,
        set: Option<Executable<'gc>>,
        attributes: EnumSet<Attribute>,
    ) {
        self.base()
            .add_property_with_case(activation, gc_context, name, get, set, attributes)
    }

    fn set_watcher(
        &self,
        activation: &mut Activation<'_, 'gc>,
        gc_context: MutationContext<'gc, '_>,
        name: Cow<str>,
        callback: Executable<'gc>,
        user_data: Value<'gc>,
    ) {
        self.base()
            .set_watcher(activation, gc_context, name, callback, user_data);
    }

    fn remove_watcher(
        &self,
        activation: &mut Activation<'_, 'gc>,
        gc_context: MutationContext<'gc, '_>,
        name: Cow<str>,
    ) -> bool {
        self.base().remove_watcher(activation, gc_context, name)
    }

    fn has_property(
        &self,
        activation: &mut Activation<'_, 'gc>,
        context: &mut UpdateContext<'_, 'gc, '_>,
        name: &str,
    ) -> bool {
        self.base().has_property(activation, context, name)
    }

    fn has_own_property(
        &self,
        activation: &mut Activation<'_, 'gc>,
        context: &mut UpdateContext<'_, 'gc, '_>,
        name: &str,
    ) -> bool {
        self.base().has_own_property(activation, context, name)
    }

    fn has_own_virtual(
        &self,
        activation: &mut Activation<'_, 'gc>,
        context: &mut UpdateContext<'_, 'gc, '_>,
        name: &str,
    ) -> bool {
        self.base().has_own_virtual(activation, context, name)
    }

    fn is_property_enumerable(&self, activation: &mut Activation<'_, 'gc>, name: &str) -> bool {
        self.base().is_property_enumerable(activation, name)
    }

    fn get_keys(&self, activation: &mut Activation<'_, 'gc>) -> Vec<String> {
        self.base().get_keys(activation)
    }

    fn as_string(&self) -> Cow<str> {
        Cow::Owned(self.base().as_string().into_owned())
    }

    fn type_of(&self) -> &'static str {
        self.base().type_of()
    }

    fn interfaces(&self) -> Vec<Object<'gc>> {
        self.base().interfaces()
    }

    fn set_interfaces(
        &mut self,
        gc_context: MutationContext<'gc, '_>,
        iface_list: Vec<Object<'gc>>,
    ) {
        self.base().set_interfaces(gc_context, iface_list)
    }

    fn as_script_object(&self) -> Option<ScriptObject<'gc>> {
        Some(self.base())
    }

    fn as_display_object(&self) -> Option<DisplayObject<'gc>> {
        None
    }

    fn as_executable(&self) -> Option<Executable<'gc>> {
        None
    }

    fn as_sound_object(&self) -> Option<SoundObject<'gc>> {
        Some(*self)
    }

    fn as_ptr(&self) -> *const ObjectPtr {
        self.0.as_ptr() as *const ObjectPtr
    }

    fn length(&self) -> usize {
        self.base().length()
    }

    fn array(&self) -> Vec<Value<'gc>> {
        self.base().array()
    }

    fn set_length(&self, gc_context: MutationContext<'gc, '_>, length: usize) {
        self.base().set_length(gc_context, length)
    }

    fn array_element(&self, index: usize) -> Value<'gc> {
        self.base().array_element(index)
    }

    fn set_array_element(
        &self,
        index: usize,
        value: Value<'gc>,
        gc_context: MutationContext<'gc, '_>,
    ) -> usize {
        self.base().set_array_element(index, value, gc_context)
    }

    fn delete_array_element(&self, index: usize, gc_context: MutationContext<'gc, '_>) {
        self.base().delete_array_element(index, gc_context)
    }
}
