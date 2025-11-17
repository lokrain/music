use alloc::collections::btree_map::{
    Entry, IntoIter as BTreeIntoIter, Iter as BTreeIter, IterMut as BTreeIterMut,
    Keys as BTreeKeys, Values as BTreeValues, ValuesMut as BTreeValuesMut,
};
use alloc::{collections::BTreeMap, sync::Arc};
use core::{fmt, iter::FromIterator};

use crate::system::{PitchSystem, PitchSystemId};

use super::{RegistryInsertError, TuningError};

/// Registry mapping tuning IDs to concrete tuning systems.
#[derive(Clone, Default)]
pub struct TuningRegistry {
    systems: BTreeMap<PitchSystemId, Arc<dyn PitchSystem>>,
}

impl fmt::Debug for TuningRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TuningRegistry")
            .field("system_count", &self.systems.len())
            .finish()
    }
}

impl TuningRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self {
            systems: BTreeMap::new(),
        }
    }

    /// Number of registered tuning systems.
    #[must_use]
    pub fn len(&self) -> usize {
        self.systems.len()
    }

    /// Whether the registry is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.systems.is_empty()
    }

    /// Insert an [`Arc`]ed system, returning the previous entry if one existed.
    pub fn insert(
        &mut self,
        id: PitchSystemId,
        system: Arc<dyn PitchSystem>,
    ) -> Option<Arc<dyn PitchSystem>> {
        self.systems.insert(id, system)
    }

    pub fn register(&mut self, id: PitchSystemId, system: Arc<dyn PitchSystem>) {
        let _ = self.insert(id, system);
    }

    /// Register a concrete [`PitchSystem`] without manually wrapping it in [`Arc`].
    pub fn register_system<T>(&mut self, id: impl Into<PitchSystemId>, system: T)
    where
        T: PitchSystem + 'static,
    {
        let _ = self.insert(id.into(), Arc::new(system));
    }

    /// Attempt to register a system, returning an error if the identifier is already in use.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryInsertError::DuplicateSystem`] when the provided identifier already
    /// exists within the registry.
    pub fn try_register_system<T>(
        &mut self,
        id: impl Into<PitchSystemId>,
        system: T,
    ) -> Result<(), RegistryInsertError>
    where
        T: PitchSystem + 'static,
    {
        let id = id.into();
        if self.systems.contains_key(&id) {
            return Err(RegistryInsertError::DuplicateSystem(id));
        }
        self.register_system(id, system);
        Ok(())
    }

    /// Register a system if the identifier is currently unused.
    ///
    /// Returns `true` when the system was inserted.
    pub fn register_if_absent<T>(&mut self, id: impl Into<PitchSystemId>, system: T) -> bool
    where
        T: PitchSystem + 'static,
    {
        let id = id.into();
        if self.systems.contains_key(&id) {
            return false;
        }
        self.systems.insert(id, Arc::new(system));
        true
    }

    /// Builder-style helper returning `self` after registering a system.
    #[must_use]
    pub fn with_system<T>(mut self, id: impl Into<PitchSystemId>, system: T) -> Self
    where
        T: PitchSystem + 'static,
    {
        self.register_system(id, system);
        self
    }

    #[must_use]
    pub fn get(&self, id: &PitchSystemId) -> Option<&Arc<dyn PitchSystem>> {
        self.systems.get(id)
    }

    /// Borrow-based lookup using `&str` identifiers.
    #[must_use]
    pub fn get_str(&self, id: &str) -> Option<&Arc<dyn PitchSystem>> {
        self.systems.get(id)
    }

    /// Check whether a system is registered.
    #[must_use]
    pub fn contains(&self, id: &PitchSystemId) -> bool {
        self.systems.contains_key(id)
    }

    /// Borrow-based containment check using `&str` identifiers.
    #[must_use]
    pub fn contains_str(&self, id: &str) -> bool {
        self.systems.contains_key(id)
    }

    /// Iterate over registered identifiers only.
    pub fn ids(&self) -> BTreeKeys<'_, PitchSystemId, Arc<dyn PitchSystem>> {
        self.systems.keys()
    }

    /// Iterate over registered systems only.
    pub fn systems(&self) -> BTreeValues<'_, PitchSystemId, Arc<dyn PitchSystem>> {
        self.systems.values()
    }

    /// Mutable iterator over registered systems.
    pub fn systems_mut(&mut self) -> BTreeValuesMut<'_, PitchSystemId, Arc<dyn PitchSystem>> {
        self.systems.values_mut()
    }

    /// Remove a system by its identifier.
    pub fn remove(&mut self, id: &PitchSystemId) -> Option<Arc<dyn PitchSystem>> {
        self.systems.remove(id)
    }

    /// Remove a system by borrowed `&str` identifier.
    pub fn remove_str(&mut self, id: &str) -> Option<Arc<dyn PitchSystem>> {
        self.systems.remove(id)
    }

    /// Remove all registered systems.
    pub fn clear(&mut self) {
        self.systems.clear();
    }

    /// Iterate over registered systems in deterministic order.
    pub fn iter(&self) -> impl Iterator<Item = (&PitchSystemId, &Arc<dyn PitchSystem>)> {
        self.systems.iter()
    }

    /// Mutable iterator over registered systems when callers need to tweak state in-place.
    pub fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = (&PitchSystemId, &mut Arc<dyn PitchSystem>)> {
        self.systems.iter_mut()
    }

    /// Insert a system lazily, returning a shared reference to the stored [`Arc`].
    pub fn get_or_insert_with<T, F>(
        &mut self,
        id: impl Into<PitchSystemId>,
        factory: F,
    ) -> &Arc<dyn PitchSystem>
    where
        T: PitchSystem + 'static,
        F: FnOnce() -> T,
    {
        match self.systems.entry(id.into()) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(Arc::new(factory())),
        }
    }

    /// Consume the registry, yielding the underlying `(id, system)` pairs.
    pub fn into_entries(self) -> impl Iterator<Item = (PitchSystemId, Arc<dyn PitchSystem>)> {
        self.into_iter()
    }

    /// Resolve a system reference, returning a helpful error when missing.
    ///
    /// # Errors
    ///
    /// Returns [`TuningError::UnknownSystem`] when the requested identifier is absent.
    pub fn resolve_system(&self, id: &PitchSystemId) -> Result<&Arc<dyn PitchSystem>, TuningError> {
        self.get(id)
            .ok_or_else(|| TuningError::UnknownSystem(id.clone()))
    }

    /// Borrow-based resolution helper using `&str` identifiers.
    ///
    /// # Errors
    ///
    /// Returns [`TuningError::UnknownSystem`] when the supplied identifier is absent.
    pub fn resolve_system_str(&self, id: &str) -> Result<&Arc<dyn PitchSystem>, TuningError> {
        self.systems
            .get(id)
            .ok_or_else(|| TuningError::UnknownSystem(PitchSystemId::from(id)))
    }

    /// Resolve an abstract pitch into a literal frequency.
    ///
    /// # Errors
    ///
    /// Returns [`TuningError::UnknownSystem`] when the system identifier has not been registered.
    pub fn resolve_frequency(&self, id: &PitchSystemId, index: i32) -> Result<f32, TuningError> {
        self.resolve_system(id)
            .map(|system| system.to_frequency(index))
    }

    /// Borrow-based helper mirroring [`Self::resolve_frequency`] for `&str` identifiers.
    ///
    /// # Errors
    ///
    /// Returns [`TuningError::UnknownSystem`] when the provided identifier cannot be resolved.
    pub fn resolve_frequency_str(&self, id: &str, index: i32) -> Result<f32, TuningError> {
        self.resolve_system_str(id)
            .map(|system| system.to_frequency(index))
    }

    /// Resolve the optional symbolic name provided by the system.
    ///
    /// # Errors
    ///
    /// Returns [`TuningError::UnknownSystem`] when the target system is unknown.
    pub fn resolve_name(
        &self,
        id: &PitchSystemId,
        index: i32,
    ) -> Result<Option<String>, TuningError> {
        self.resolve_system(id).map(|system| system.name_of(index))
    }

    /// Borrow-based variant of [`Self::resolve_name`].
    ///
    /// # Errors
    ///
    /// Returns [`TuningError::UnknownSystem`] when the identifier has not been registered.
    pub fn resolve_name_str(&self, id: &str, index: i32) -> Result<Option<String>, TuningError> {
        self.resolve_system_str(id)
            .map(|system| system.name_of(index))
    }

    #[must_use]
    pub fn to_frequency(&self, id: &PitchSystemId, index: i32) -> Option<f32> {
        self.resolve_frequency(id, index).ok()
    }

    #[must_use]
    pub fn name_of(&self, id: &PitchSystemId, index: i32) -> Option<String> {
        self.resolve_name(id, index).ok().flatten()
    }
}

impl FromIterator<(PitchSystemId, Arc<dyn PitchSystem>)> for TuningRegistry {
    fn from_iter<I: IntoIterator<Item = (PitchSystemId, Arc<dyn PitchSystem>)>>(iter: I) -> Self {
        let mut registry = Self::new();
        registry.extend(iter);
        registry
    }
}

impl Extend<(PitchSystemId, Arc<dyn PitchSystem>)> for TuningRegistry {
    fn extend<I: IntoIterator<Item = (PitchSystemId, Arc<dyn PitchSystem>)>>(&mut self, iter: I) {
        for (id, system) in iter {
            let _ = self.insert(id, system);
        }
    }
}

impl IntoIterator for TuningRegistry {
    type Item = (PitchSystemId, Arc<dyn PitchSystem>);
    type IntoIter = BTreeIntoIter<PitchSystemId, Arc<dyn PitchSystem>>;

    fn into_iter(self) -> Self::IntoIter {
        self.systems.into_iter()
    }
}

impl<'a> IntoIterator for &'a TuningRegistry {
    type Item = (&'a PitchSystemId, &'a Arc<dyn PitchSystem>);
    type IntoIter = BTreeIter<'a, PitchSystemId, Arc<dyn PitchSystem>>;

    fn into_iter(self) -> Self::IntoIter {
        self.systems.iter()
    }
}

impl<'a> IntoIterator for &'a mut TuningRegistry {
    type Item = (&'a PitchSystemId, &'a mut Arc<dyn PitchSystem>);
    type IntoIter = BTreeIterMut<'a, PitchSystemId, Arc<dyn PitchSystem>>;

    fn into_iter(self) -> Self::IntoIter {
        self.systems.iter_mut()
    }
}
