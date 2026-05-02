/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Reusable Batch Data Conversion
//!
//! Provides `DataConverters`, a lightweight iterator adapter used to convert
//! batches of common runtime values with the single-value [`DataConverter`]
//! rules.
//!
//! # Author
//!
//! Haixing Hu

use std::marker::PhantomData;

use super::data_conversion_error::DataConversionError;
use super::data_conversion_result::DataConversionResult;
use super::data_convert_to::DataConvertTo;
use super::data_converter::DataConverter;
use super::data_list_conversion_error::DataListConversionError;
use super::data_list_conversion_result::DataListConversionResult;

/// A lightweight adapter for converting batches of source values.
///
/// `DataConverters` stores an iterator and converts each item through
/// [`DataConverter`]. Borrowed inputs such as `&Vec<T>` and `&[T]` are converted
/// by reference and do not clone the source collection.
///
/// # Author
///
/// Haixing Hu
#[derive(Debug, Clone)]
pub struct DataConverters<'a, I> {
    /// The iterator of source values.
    sources: I,
    /// The marker for the lifetime of the iterator.
    marker: PhantomData<&'a ()>,
}

impl<'a, I> DataConverters<'a, I>
where
    I: Iterator,
{
    /// Creates a batch converter from an iterator.
    ///
    /// # Parameters
    ///
    /// * `sources` - Iterator that yields values convertible to
    ///   [`DataConverter`].
    ///
    /// # Returns
    ///
    /// Returns a batch converter that consumes the iterator when conversion is
    /// requested.
    #[inline]
    pub fn from_iterator(sources: I) -> Self {
        Self {
            sources,
            marker: PhantomData,
        }
    }
}

impl<'a, I> DataConverters<'a, I>
where
    I: Iterator,
    I::Item: Into<DataConverter<'a>>,
{
    /// Converts every source item to the requested target type.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Target element type.
    ///
    /// # Returns
    ///
    /// Returns converted values in source order. Empty sources return an empty
    /// vector.
    ///
    /// # Errors
    ///
    /// Returns [`DataListConversionError`] with the zero-based failing index and
    /// the original [`DataConversionError`] when any element fails conversion.
    pub fn to_vec<T>(self) -> DataListConversionResult<Vec<T>>
    where
        DataConverter<'a>: DataConvertTo<T>,
    {
        let (capacity, _) = self.sources.size_hint();
        let mut converted = Vec::with_capacity(capacity);
        for (index, source) in self.sources.enumerate() {
            let value = source
                .into()
                .to::<T>()
                .map_err(|e| DataListConversionError { index, source: e })?;
            converted.push(value);
        }
        Ok(converted)
    }

    /// Converts the first source item to the requested target type.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Target type.
    ///
    /// # Returns
    ///
    /// Returns the converted first value.
    ///
    /// # Errors
    ///
    /// Returns [`DataConversionError::NoValue`] when the source iterator is
    /// empty, or the original single-value conversion error when the first
    /// element cannot be converted.
    pub fn to_first<T>(mut self) -> DataConversionResult<T>
    where
        DataConverter<'a>: DataConvertTo<T>,
    {
        match self.sources.next() {
            Some(source) => source.into().to::<T>(),
            None => Err(DataConversionError::NoValue),
        }
    }
}

impl<'a, I> DataConverters<'a, I>
where
    I: ExactSizeIterator,
{
    /// Returns the number of source items remaining in this converter.
    ///
    /// # Returns
    ///
    /// Returns the exact number of items that can still be converted.
    #[inline]
    pub fn len(&self) -> usize {
        self.sources.len()
    }

    /// Returns whether this converter has no source items remaining.
    ///
    /// # Returns
    ///
    /// Returns `true` when [`Self::len`] is zero.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.sources.len() == 0
    }
}

impl<'a, V> From<&'a [V]> for DataConverters<'a, std::slice::Iter<'a, V>>
where
    &'a V: Into<DataConverter<'a>>,
{
    /// Creates a batch converter from a borrowed slice.
    #[inline]
    fn from(values: &'a [V]) -> Self {
        Self::from_iterator(values.iter())
    }
}

impl<'a, V> From<&'a Vec<V>> for DataConverters<'a, std::slice::Iter<'a, V>>
where
    &'a V: Into<DataConverter<'a>>,
{
    /// Creates a batch converter from a borrowed vector.
    #[inline]
    fn from(values: &'a Vec<V>) -> Self {
        Self::from(values.as_slice())
    }
}

impl<V> From<Vec<V>> for DataConverters<'static, std::vec::IntoIter<V>>
where
    V: Into<DataConverter<'static>>,
{
    /// Creates a batch converter from an owned vector.
    #[inline]
    fn from(values: Vec<V>) -> Self {
        Self::from_iterator(values.into_iter())
    }
}
