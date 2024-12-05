package com.armatak.simtak.trackerLog.data.viewHolder

import android.view.View
import androidx.recyclerview.widget.RecyclerView.ViewHolder
import com.armatak.simtak.R
import com.armatak.simtak.databinding.ItemLogTrackerBinding
import com.armatak.simtak.trackerLog.data.models.LogModel
import com.armatak.simtak.trackerLog.data.models.LogTypes

class ViewHolderLogTracker(view: View) : ViewHolder(view) {
    private val binding = ItemLogTrackerBinding.bind(view)

    fun render(item: LogModel) {
        val tvLogBody = binding.tvLogBody
        val tvLogType = binding.tvLogType
        binding.tvLogTime.text = item.time
        tvLogBody.text = item.body

        tvLogType.apply {
            when (item.type) {
                LogTypes.Error -> {
                    tvLogBody.setTextColor(context.getColor(R.color.errorColor))
                    setTextColor(context.getColor(R.color.errorColor))
                    text = context.getString(R.string.error)
                }

                LogTypes.NetworkOperation -> {
                    tvLogBody.setTextColor(context.getColor(R.color.networkColor))
                    setTextColor(context.getColor(R.color.networkColor))
                    text = context.getString(R.string.networkOperation)
                }
                LogTypes.Normal -> {
                    tvLogBody.setTextColor(context.getColor(R.color.darkGrey))
                    setTextColor(context.getColor(R.color.darkGrey))
                    text = context.getString(R.string.normal)
                }
                LogTypes.Warning -> {
                    tvLogBody.setTextColor(context.getColor(R.color.warningColor))
                    setTextColor(context.getColor(R.color.warningColor))
                    text = context.getString(R.string.warning)
                }
            }
        }
    }
}