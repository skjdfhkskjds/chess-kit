const WIDTH: usize = 40;
const HEIGHT: usize = 40;

/// sample maps a normalized silhouette into a target pixel canvas.
///
/// @param: mask - normalized piece silhouette
/// @param: x - horizontal pixel coordinate
/// @param: y - vertical pixel coordinate
/// @param: bounds - silhouette origin and scaled dimensions
/// @return: whether the sampled pixel belongs to the piece
pub(super) fn sample(mask: &str, x: u16, y: u16, bounds: (u16, u16, u16, u16)) -> bool {
    let (left, top, width, height) = bounds;
    if x < left || x >= left + width || y < top || y >= top + height {
        return false;
    }

    let mask_x = scale_coordinate(usize::from(x - left), usize::from(width), WIDTH);
    let mask_y = scale_coordinate(usize::from(y - top), usize::from(height), HEIGHT);
    let stride = mask.find('\n').map_or(WIDTH, |newline| newline + 1);
    mask.as_bytes()[mask_y * stride + mask_x] == b'#'
}

/// Maps a target coordinate across the full source range using rounded sampling.
fn scale_coordinate(coordinate: usize, target_size: usize, source_size: usize) -> usize {
    if target_size <= 1 {
        return 0;
    }

    (coordinate * (source_size - 1) + (target_size - 1) / 2) / (target_size - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordinate_scaling_preserves_both_endpoints_and_symmetry() {
        let coordinates = (0..12)
            .map(|coordinate| scale_coordinate(coordinate, 12, WIDTH))
            .collect::<Vec<_>>();

        assert_eq!(coordinates.first(), Some(&0));
        assert_eq!(coordinates.last(), Some(&(WIDTH - 1)));
        assert!(
            coordinates
                .iter()
                .zip(coordinates.iter().rev())
                .all(|(left, right)| left + right == WIDTH - 1)
        );
    }
}
